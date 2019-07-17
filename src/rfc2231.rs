//! [Encoded MIME parameters]
//!
//! Implements [RFC 2045] syntax extended with RFC 2231
//!
//! [Encoded MIME parameters]: https://tools.ietf.org/html/rfc2231
//! [RFC 2045]: https://tools.ietf.org/html/rfc2045


use std::borrow::Cow;
use std::fmt::{self, Display};
use std::str;
use std::collections::HashMap;

use encoding::label::encoding_from_whatwg_label;
use encoding::types::EncodingRef;
use encoding::DecoderTrap;
use encoding::all::ASCII;
use nom::character::is_digit;
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{pair, preceded, separated_pair, terminated, tuple};

use crate::util::*;
use crate::rfc3461::hexpair;
use crate::rfc5234::crlf;
use crate::rfc5322::{ofws, quoted_string};

#[derive(Debug)]
struct Parameter<'a> {
    name: Name<'a>,
    value: Value<'a>,
}

#[derive(Debug)]
struct Name<'a> {
    section: Option<u32>,
    name: &'a str,
}

#[derive(Debug)]
enum Value<'a> {
    Regular(Cow<'a, str>),
    Extended(ExtendedValue<'a>),
}

#[derive(Debug)]
enum ExtendedValue<'a> {
    Initial { encoding: Option<&'a [u8]>, language: Option<&'a [u8]>, value: Vec<u8> },
    Other(Vec<u8>),
}

named!(_equals<CBS, ()>,
    do_parse!(
        ofws >>
        tag!("=") >>
        ofws >>
        ()
    )
);

fn parameter(input: &[u8]) -> NomResult<Parameter> {
    alt((regular_parameter, extended_parameter))(input)
}

fn regular_parameter(input: &[u8]) -> NomResult<Parameter> {
    map(separated_pair(regular_parameter_name, _equals, value),
        |(name, value)| Parameter{name, value: Value::Regular(value)})(input)
}

fn regular_parameter_name(input: &[u8]) -> NomResult<Name> {
    map(pair(attribute, opt(section)),
        |(name, section)| Name{name: std::str::from_utf8(name).unwrap(), section}
    )(input)
}

named!(token<CBS, &str>,
    map!(take_while1!(|c| (33..=126).contains(&c) && !b"()<>@,;:\\\"/[]?=".contains(&c)),
         |t| std::str::from_utf8(&t).unwrap()
    )
);

fn is_attribute_char(c: u8) -> bool {
    (33..=126).contains(&c) && !b"*'%()<>@,;:\\\"/[]?=".contains(&c)
}

#[inline]
named!(attribute_char<CBS, u8>,
    map!(verify!(take!(1), |x: CBS| is_attribute_char(x[0])), |c| c[0])
);

named!(attribute<CBS, CBS>,
    take_while1!(|c| is_attribute_char(c))
);

named!(section<CBS, u32>,
    alt!(initial_section | other_sections)
);

named!(initial_section<CBS, u32>,
    do_parse!(tag!("*0") >> (0))
);

named!(other_sections<CBS, u32>,
    do_parse!(
        tag!("*") >>
        s: verify!(take_while_m_n!(1, 9, is_digit), |x: CBS| x[0] != b'0') >>
        (str::from_utf8(&s).unwrap().parse().unwrap())
    )
);

fn extended_parameter(input: &[u8]) -> NomResult<Parameter> {
    alt((
        map(separated_pair(extended_initial_name,
                           _equals,
                           extended_initial_value),
            |(name, value)| Parameter{name, value: Value::Extended(value)}),

        map(separated_pair(extended_other_names,
                           _equals,
                           extended_other_values),
            |(name, value)| Parameter{name, value: Value::Extended(ExtendedValue::Other(value))}),
   ))(input)
}

named!(extended_initial_name<CBS, Name>,
    do_parse!(
        name: attribute >>
        section: opt!(initial_section) >>
        tag!("*") >>
        (Name{name: str::from_utf8(&name).unwrap(), section})
    )
);

named!(extended_other_names<CBS, Name>,
    do_parse!(
        name: attribute >>
        section: other_sections >>
        tag!("*") >>
        (Name{name: str::from_utf8(&name).unwrap(), section: Some(section)})
    )
);

fn extended_initial_value(input: &[u8]) -> NomResult<ExtendedValue> {
    let (rem, (e, l, v)) = tuple((
        terminated(opt(attribute), tag("'")),
        terminated(opt(attribute), tag("'")),
        extended_other_values
    ))(input)?;

    Ok((rem, ExtendedValue::Initial{encoding: e, language: l, value: v}))
}

named!(ext_octet<CBS, u8>,
    do_parse!(tag!("%") >> h: hexpair >> (h))
);

named!(extended_other_values<CBS, Vec<u8>>,
    many0!(alt!(ext_octet | attribute_char))
);

named!(value<CBS, Cow<'_, str>>,
   alt!(map!(token, Cow::from) | map!(quoted_string, |qs| Cow::from(qs.0)))
);


named!(_mime_type<CBS, CBS>,
    recognize!(do_parse!(token >> tag!("/") >> token >> ()))
);

fn _parameter_list(input: &[u8]) -> NomResult<Vec<Parameter>> {
    terminated(many0(preceded(pair(tag(";"), ofws), parameter)),
               pair(opt(tag(";")), opt(crlf)))(input)
}

#[derive(Debug)]
enum Segment<'a> {
    Encoded(Vec<u8>),
    Decoded(Cow<'a, str>),
}

fn decode_segments(mut input: Vec<(u32, Segment)>, encoding: EncodingRef) -> String {
    input.sort_by(|a, b| a.0.cmp(&b.0));
    let mut out = String::new();
    let mut encoded = Vec::new();

    let decode = |bytes: &mut Vec<_>, out: &mut String| {
        out.push_str(&encoding.decode(&bytes, DecoderTrap::Replace).unwrap());
        bytes.clear();
    };

    // Clump encoded segments together before decoding. Prevents partial UTF-8 sequences or similar with other encodings.
    for (_, segment) in input {
        match segment {
            Segment::Encoded(mut bytes) => encoded.append(&mut bytes),
            Segment::Decoded(s) => { decode(&mut encoded, &mut out); out.push_str(&s) }
        }
    }
    decode(&mut encoded, &mut out);

    out
}

fn decode_parameter_list(input: Vec<Parameter>) -> Vec<(String, String)> {
    let mut simple = HashMap::<String, String>::new();
    let mut simple_encoded = HashMap::<String, String>::new();
    let mut composite = HashMap::<String, Vec<(u32, Segment)>>::new();
    let mut composite_encoding = HashMap::new();

    for Parameter{name, value} in input {
        let name_norm = name.name.to_lowercase();

        match name.section {
            None => {
                match value {
                    Value::Regular(v) => { simple.insert(name_norm, v.into()); },
                    Value::Extended(ExtendedValue::Initial{value, encoding: encoding_name, ..}) => {
                        let codec = match encoding_name {
                            Some(encoding_name) => encoding_from_whatwg_label(&ascii_to_string(&encoding_name)).unwrap_or(ASCII),
                            None => ASCII,
                        };
                        simple_encoded.insert(name_norm, codec.decode(&value, DecoderTrap::Replace).unwrap());
                    }
                    Value::Extended(ExtendedValue::Other(..)) => unreachable!(),
                }
            },
            Some(section) => {
                let ent = composite.entry(name_norm.clone()).or_default();

                match value {
                    Value::Regular(v) => ent.push((section, Segment::Decoded(v))),
                    Value::Extended(ExtendedValue::Initial{value, encoding: encoding_name, ..}) => {
                        if let Some(encoding_name) = encoding_name {
                            if let Some(codec) = encoding_from_whatwg_label(&ascii_to_string(&encoding_name)) {
                                composite_encoding.insert(name_norm, codec);
                            }
                        }
                        ent.push((section, Segment::Encoded(value.to_vec())))
                    }
                    Value::Extended(ExtendedValue::Other(v)) => ent.push((section, Segment::Encoded(v))),
                }
            }
        }
    }

    let mut composite_out = Vec::new();
    for (name, segments) in composite {
        let codec = composite_encoding.get(&name).cloned().unwrap_or(ASCII);
        composite_out.push((name, decode_segments(segments, codec)));
    }

    for (name, value) in simple_encoded.into_iter().chain(composite_out.into_iter()) {
        simple.insert(name, value);
    }

    simple.into_iter().collect()
}

/// Parse a MIME `"Content-Type"` header.
///
/// Returns a tuple of the MIME type and parameters.
named!(pub content_type<CBS, (String, Vec<(String, String)>)>,
    do_parse!(
        ofws >>
        mt: _mime_type >>
        ofws >>
        p: _parameter_list >>
        (ascii_to_string(mt).to_lowercase(), decode_parameter_list(p))
    )
);


named!(_x_token<CBS, &str>,
    do_parse!(
        tag_no_case!("x-") >>
        token: token >>
        (token)
    )
);

/// Value from a MIME `"Content-Disposition"` header.
#[derive(Debug, PartialEq)]
pub enum ContentDisposition {
    /// "inline"
    Inline,
    /// "attachment"
    Attachment,
    /// Value prefixed with "X-". The prefix is not stored in the
    /// string.
    Extended(String),
    /// Any syntaxically valid token that is not any known disposition.
    Token(String),
}

impl Display for ContentDisposition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContentDisposition::Inline => write!(f, "inline"),
            ContentDisposition::Attachment => write!(f, "attachment"),
            ContentDisposition::Extended(s) => write!(f, "x-{}", s),
            ContentDisposition::Token(t) => write!(f, "{}", t),
        }
    }
}

named!(_disposition<CBS, ContentDisposition>,
    alt!(
        map!(tag_no_case!("inline"), |_| ContentDisposition::Inline) |
        map!(tag_no_case!("attachment"), |_| ContentDisposition::Attachment) |
        map!(_x_token, |x| ContentDisposition::Extended(x.into())) |
        map!(token, |t| ContentDisposition::Token(t.into()))
    )
);

/// Parse a MIME `"Content-Disposition"` header.
///
/// Returns a tuple of [`ContentDisposition`] and parameters.
named!(pub content_disposition<CBS, (ContentDisposition, Vec<(String, String)>)>,
    do_parse!(
        ofws >>
        disp: _disposition >>
        ofws >>
        p: _parameter_list >>
        (disp, decode_parameter_list(p))
    )
);

/// Value from a MIME `"Content-Transfer-Encoding"` header.
#[derive(Debug, PartialEq)]
pub enum ContentTransferEncoding {
    /// "7bit"
    SevenBit,
    /// "8bit"
    EightBit,
    /// "binary"
    Binary,
    /// "base64"
    Base64,
    /// "quoted-printable"
    QuotedPrintable,
    /// Value prefixed with "X-". The prefix is not stored in the
    /// string.
    Extended(String),
    /// Any syntaxically valid token that is not any known encoding.
    Token(String),
}

impl Display for ContentTransferEncoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CTE::SevenBit => write!(f, "7bit"),
            CTE::EightBit => write!(f, "8bit"),
            CTE::Binary => write!(f, "binary"),
            CTE::Base64 => write!(f, "base64"),
            CTE::QuotedPrintable => write!(f, "quoted-printable"),
            CTE::Extended(s) => write!(f, "x-{}", s),
            CTE::Token(t) => write!(f, "{}", t),
        }
    }
}

use self::ContentTransferEncoding as CTE;

/// Parse a MIME `"Content-Transfer-Encoding"` header.
///
/// Returns a [`ContentTransferEncoding`].
named!(pub content_transfer_encoding<CBS, ContentTransferEncoding>,
    do_parse!(
        ofws >>
        cte: alt!(
            map!(tag_no_case!("7bit"), |_| CTE::SevenBit) |
            map!(tag_no_case!("8bit"), |_| CTE::EightBit) |
            map!(tag_no_case!("binary"), |_| CTE::Binary) |
            map!(tag_no_case!("base64"), |_| CTE::Base64) |
            map!(tag_no_case!("quoted-printable"), |_| CTE::QuotedPrintable) |
            map!(_x_token, |x| CTE::Extended(x.into())) |
            map!(token, |t| CTE::Token(t.into()))
        ) >>
        ofws >>
        (cte)
    )
);
