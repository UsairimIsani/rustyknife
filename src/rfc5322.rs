//! Parsers for Internet Message Format messages.
//!
//! Comments are ignored. RFC2047 decoding is applied where appropriate.

use rfc2047::encoded_word;
use rfc5234::*;
use util::*;

named!(quoted_pair<CBS, CBS>,
       do_parse!(
           tag!("\\") >>
           v: alt!(vchar | wsp) >> (v)
       )
);

named!(ctext<CBS, CBS>,
       take_while1!(|c: u8| (33..=39).contains(&c) || (42..=91).contains(&c) || (93..=126).contains(&c))
);

#[derive(Clone, Debug)]
enum CommentContent {
    Text(Vec<u8>),
    Comment(Vec<CommentContent>),
}

named!(ccontent<CBS, CommentContent>,
       alt!(map!(alt!(ctext | quoted_pair), |x| CommentContent::Text(x.0.to_vec())) | map!(comment, |y| CommentContent::Comment(y)))
);

named!(fws<CBS, Vec<u8>>,
    map!(pair!(opt!(do_parse!(
        a: many0!(wsp) >>
            crlf >> //CRLF is "semantically invisible"
        (a)
    )), many1!(wsp)), |(a, b)| {
        a.unwrap_or(vec![]).iter().chain(b.iter()).flat_map(|i| i.0.iter().cloned()).collect()
    })
);


named!(pub ofws<CBS, Vec<u8>>,
       map!(opt!(fws), |i| i.unwrap_or(Vec::new()))
);

fn _concat_comment(comments: &Vec<CommentContent>) -> Vec<CommentContent> {
    let mut out = Vec::new();
    let mut prev_text = false;

    for comment in comments {
        let (is_text, val) = match comment {
            CommentContent::Text(text) => {
                if text.is_empty() {
                    continue;
                }
                if prev_text {
                    if let Some(CommentContent::Text(mut pt)) = out.pop() {
                        pt.extend(text);
                        (true, CommentContent::Text(pt))
                    } else {
                        continue;
                    }
                } else {
                    (true, comment.clone())
                }
            }
            CommentContent::Comment(cmt) => {
                (false, CommentContent::Comment(cmt.clone()))
            },
        };
        prev_text = is_text;
        out.push(val);
    }

    out
}

named!(comment<CBS, Vec<CommentContent>>,
    do_parse!(
        tag!("(") >>
        a: fold_many0!(tuple!(ofws, ccontent), Vec::new(), |mut acc: Vec<_>, (fws, cc)| {
            acc.push(CommentContent::Text(fws));
            acc.push(cc);
            acc
        }) >>
        b: ofws >>
        tag!(")") >>
        ({let mut out = a.clone(); out.push(CommentContent::Text(b)); _concat_comment(&out)})
    )
);

named!(cfws<CBS, CBS>,
    alt!(recognize!(pair!(many1!(pair!(opt!(fws), comment)), opt!(fws))) | recognize!(fws))
);

named!(qtext<CBS, CBS>,
    take_while1!(|c: u8| c == 33 || (35..=91).contains(&c) || (93..=126).contains(&c))
);

#[cfg(feature = "quoted-string-rfc2047")]
named!(qcontent<CBS, QContent>,
    alt!(map!(encoded_word, |x| QContent::EncodedWord(x)) |
         map!(qtext, |x| QContent::Literal(ascii_to_string(x.0))) |
         map!(quoted_pair, |x| QContent::Literal(ascii_to_string(x.0)))
    )
);

#[cfg(not(feature = "quoted-string-rfc2047"))]
named!(qcontent<CBS, QContent>,
    alt!(map!(qtext, |x| QContent::Literal(ascii_to_string(x.0))) |
         map!(quoted_pair, |x| QContent::Literal(ascii_to_string(x.0)))
    )
);

fn _concat_qcontent(input: &Vec<QContent>) -> String
{
    let mut out = String::new();
    for (i, t1) in input.iter().enumerate() {
        match (t1, input.get(i+1)) {
            (QContent::Literal(v), Some(QContent::Literal(_))) => out.extend(v.chars()),
            #[cfg(feature = "quoted-string-rfc2047")]
            (QContent::EncodedWord(v), Some(QContent::EncodedWord(_))) => out.extend(v.chars()),
            #[cfg(feature = "quoted-string-rfc2047")]
            (_, Some(_)) => { out.extend(t1.get().chars()); out.push(' ') },
            (_, None) => { out.extend(t1.get().chars()) },
        }
    }

    out
}

named!(quoted_string<CBS, Vec<QContent>>,
    do_parse!(
        opt!(cfws) >>
        tag!("\"") >>
        a: many0!(tuple!(opt!(fws), qcontent)) >>
        b: opt!(fws) >>
        tag!("\"") >>
        opt!(cfws) >>
        ({
            let mut out = Vec::with_capacity(a.len()*2+1);
            for (ws, cont) in a {
                ws.map(|x| out.push(QContent::Literal(ascii_to_string(&x))));
                out.push(cont);
            }
            b.map(|x| out.push(QContent::Literal(ascii_to_string(&x))));
            out
        })
    )
);

#[derive(Clone, Debug)]
pub struct Mailbox {
    pub dname: Option<String>,
    pub address: String,
}

#[derive(Clone, Debug)]
pub struct Group {
    pub dname: String,
    pub members: Vec<Mailbox>,
}

#[derive(Clone, Debug)]
pub enum Address {
    Mailbox(Mailbox),
    Group(Group),
}

#[derive(Clone, Debug)]
enum Word {
    EncodedWord(String),
    Atom(String),
    QS(Vec<QContent>),
}

#[derive(Clone, Debug)]
enum QContent {
    Literal(String),
    #[cfg(feature = "quoted-string-rfc2047")]
    EncodedWord(String),
}

impl QContent {
    fn get(&self) -> &str {
        match self {
            QContent::Literal(s) => s,
            #[cfg(feature = "quoted-string-rfc2047")]
            QContent::EncodedWord(s) => s,
        }
    }
}
#[derive(Clone, Debug)]
enum Text {
    EncodedWord(String),
    Literal(String),
    Atom(String),
}

impl Text {
    fn get(&self) -> &str {
        match self {
            Text::EncodedWord(s) => s,
            Text::Literal(s) => s,
            Text::Atom(s) => s,
        }
    }
}

trait ToTextVec {
    fn to_text_vec(&self) -> Vec<Text>;
}

impl ToTextVec for QContent {
    fn to_text_vec(&self) -> Vec<Text> {
        match self {
            QContent::Literal(lit) => vec![Text::Literal(lit.clone())],
            #[cfg(feature = "quoted-string-rfc2047")]
            QContent::EncodedWord(ew) => vec![Text::EncodedWord(ew.clone())],
        }
    }
}

impl ToTextVec for Word {
    fn to_text_vec(&self) -> Vec<Text> {
        match self {
            Word::Atom(a) => vec![Text::Atom(a.clone())],
            Word::EncodedWord(ew) => vec![Text::EncodedWord(ew.clone())],
            Word::QS(qc) => qc.iter().map(|x| x.to_text_vec()).collect::<Vec<Vec<_>>>().iter().flat_map(|y| y.iter()).cloned().collect(),
        }
    }

}
named!(atext<CBS, CBS>,
    take_while1!(|c: u8| b"!#$%&'*+-/=?^_`{|}~".contains(&c) || (b'0'..=b'9').contains(&c) || (b'A'..=b'Z').contains(&c) || (b'a'..=b'z').contains(&c))
);

named!(dot_atom<CBS, CBS>,
    do_parse!(
        opt!(cfws) >>
        a: recognize!(pair!(atext, many0!(pair!(tag!("."), atext)))) >>
        opt!(cfws) >>
        (a)
    )
);

named!(pub atom<CBS, CBS>,
    do_parse!(
        opt!(cfws) >>
        a: atext >>
        opt!(cfws) >>
        (a)
    )
);

named!(_padded_encoded_word<CBS, String>,
    do_parse!(opt!(cfws) >> e: encoded_word >> opt!(cfws) >> (e))
);

named!(word<CBS, Word>,
    alt!(
        map!(_padded_encoded_word, |x| Word::EncodedWord(x)) |
        map!(atom, |x| Word::Atom(ascii_to_string(x.0))) |
        map!(quoted_string, |x| Word::QS(x))
    )
);

fn _concat_atom_and_qs(input: &Vec<Word>) -> String {

    let flat : Vec<Text> = input.iter().map(|item| item.to_text_vec()).collect::<Vec<Vec<_>>>()
        .iter().flat_map(|item| item.iter()).cloned().collect();

    let mut out = String::new();
    for (i, t1) in flat.iter().enumerate() {
        match (t1, flat.get(i+1)) {
            (Text::Literal(v), Some(Text::Literal(_))) => out.extend(v.chars()),
            (Text::EncodedWord(v), Some(Text::EncodedWord(_))) => out.extend(v.chars()),
            (_, Some(_)) => {out.extend(t1.get().chars()); out.push(' ')},
            (_, None) => out.extend(t1.get().chars()),
        };
    }
    out
}

named!(display_name<CBS, String>,
    map!(many1!(word), |x| _concat_atom_and_qs(&x))
);

named!(local_part<CBS, String>,
    alt!(map!(dot_atom, |x| ascii_to_string(x.0)) |
         map!(quoted_string, |x| _concat_qcontent(&x)))
);

named!(dtext<CBS, CBS>,
    take_while1!(|c: u8| (33..=90).contains(&c) || (94..=126).contains(&c))
);

named!(domain_literal<CBS, Vec<u8>>,
    do_parse!(
        opt!(cfws) >>
        tag!("[") >>
        a: many0!(tuple!(ofws, dtext)) >>
        b: ofws >>
        tag!("]") >>
        opt!(cfws) >>
        ({let mut out : Vec<u8> = vec![b'[']; out.extend(a.iter().flat_map(|(x, y)| x.iter().chain(y.0.iter()))); out.extend(b); out.push(b']'); out})
    )
);

named!(domain<CBS, Vec<u8>>,
    alt!(map!(dot_atom, |x| x.0.to_vec()) | domain_literal)
);

named!(addr_spec<CBS, String>,
    do_parse!(
        lp: local_part >>
        tag!("@") >>
        domain: domain >>
        ([&lp, "@", &ascii_to_string(&domain)].iter().flat_map(|x| x.chars()).collect())
    )
);

named!(angle_addr<CBS, String>,
    do_parse!(
        opt!(cfws) >>
        tag!("<") >>
        address: addr_spec >>
        tag!(">") >>
        opt!(cfws) >>
        (address)
    )
);

named!(name_addr<CBS, Mailbox>,
    do_parse!(
        dname: opt!(display_name) >>
        address: angle_addr >>
        (Mailbox{dname, address: address})
    )
);

named!(mailbox<CBS, Mailbox>,
    alt!(name_addr | map!(addr_spec, |a| Mailbox{dname: None, address: a}))
);

named!(mailbox_list<CBS, Vec<Mailbox>>,
    do_parse!(
        a: mailbox >>
        b: many0!(pair!(tag!(","), mailbox)) >>
        ({let mut out = vec![a]; out.extend(b.iter().map(|(_, m)| m.clone())); out})
    )
);

named!(group_list<CBS, Vec<Mailbox>>,
    alt!(mailbox_list | map!(cfws, |_| vec![]))
);

named!(group<CBS, Group>,
    do_parse!(
        dname: display_name >>
        tag!(":") >>
        members: opt!(group_list) >>
        tag!(";") >>
        opt!(cfws) >>
        (Group{dname, members: members.unwrap_or(vec![])})
    )
);

named!(address<CBS, Address>,
    alt!(map!(mailbox, |x| Address::Mailbox(x)) | map!(group, |x| Address::Group(x)))
);

named!(address_list<CBS, Vec<Address>>,
    do_parse!(
        a: address >>
        b: many0!(pair!(tag!(","), address)) >>
        ({let mut out = vec![a]; out.extend(b.iter().map(|(_, m)| m.clone())); out})
    )
);

named!(address_list_crlf<CBS, Vec<Address>>,
    do_parse!(
        a: address_list >>
        opt!(crlf) >>
        (a)
    )
);

named!(address_crlf<CBS, Address>,
    do_parse!(
        a: address >>
        opt!(crlf) >>
        (a)
    )
);

// Updated from RFC6854
pub fn from(i: &[u8]) -> KResult<&[u8], Vec<Address>> {
    wrap_cbs_result(address_list_crlf(CBS(i)))
}

pub fn sender(i: &[u8]) -> KResult<&[u8], Address> {
    wrap_cbs_result(address_crlf(CBS(i)))
}

pub fn reply_to(i: &[u8]) -> KResult<&[u8], Vec<Address>> {
    wrap_cbs_result(address_list_crlf(CBS(i)))
}