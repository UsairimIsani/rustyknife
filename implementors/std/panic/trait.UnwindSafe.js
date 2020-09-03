(function() {var implementors = {};
implementors["arrayvec"] = [{"text":"impl&lt;A&gt; UnwindSafe for ArrayString&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: UnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as Array&gt;::Index: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for CapacityError&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;A&gt; UnwindSafe for ArrayVec&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: UnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as Array&gt;::Index: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;A&gt; UnwindSafe for IntoIter&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: UnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as Array&gt;::Index: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, A&gt; UnwindSafe for Drain&lt;'a, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: RefUnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as Array&gt;::Index: RefUnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as Array&gt;::Item: RefUnwindSafe,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["base64"] = [{"text":"impl UnwindSafe for Config","synthetic":true,"types":[]},{"text":"impl UnwindSafe for DecodeError","synthetic":true,"types":[]},{"text":"impl UnwindSafe for CharacterSet","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; UnwindSafe for Base64Display&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a, W&gt; !UnwindSafe for EncoderWriter&lt;'a, W&gt;","synthetic":true,"types":[]}];
implementors["encoding"] = [{"text":"impl UnwindSafe for CodecError","synthetic":true,"types":[]},{"text":"impl UnwindSafe for DecoderTrap","synthetic":true,"types":[]},{"text":"impl UnwindSafe for EncoderTrap","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ErrorEncoding","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ErrorEncoder","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ErrorDecoder","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ASCIIEncoding","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ASCIIEncoder","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ASCIIDecoder","synthetic":true,"types":[]},{"text":"impl UnwindSafe for SingleByteEncoding","synthetic":true,"types":[]},{"text":"impl UnwindSafe for SingleByteEncoder","synthetic":true,"types":[]},{"text":"impl UnwindSafe for SingleByteDecoder","synthetic":true,"types":[]},{"text":"impl UnwindSafe for UTF8Encoding","synthetic":true,"types":[]},{"text":"impl UnwindSafe for UTF8Encoder","synthetic":true,"types":[]},{"text":"impl UnwindSafe for UTF8Decoder","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Little","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Big","synthetic":true,"types":[]},{"text":"impl&lt;E&gt; UnwindSafe for UTF16Encoding&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;E&gt; UnwindSafe for UTF16Encoder&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;E&gt; UnwindSafe for UTF16Decoder&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Windows949Encoding","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Windows949Encoder","synthetic":true,"types":[]},{"text":"impl UnwindSafe for EUCJPEncoding","synthetic":true,"types":[]},{"text":"impl UnwindSafe for EUCJPEncoder","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Windows31JEncoding","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Windows31JEncoder","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ISO2022JPEncoding","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ISO2022JPEncoder","synthetic":true,"types":[]},{"text":"impl UnwindSafe for GBK","synthetic":true,"types":[]},{"text":"impl UnwindSafe for GB18030","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for GBEncoding&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for GBEncoder&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl UnwindSafe for HZEncoding","synthetic":true,"types":[]},{"text":"impl UnwindSafe for HZEncoder","synthetic":true,"types":[]},{"text":"impl UnwindSafe for BigFive2003Encoding","synthetic":true,"types":[]},{"text":"impl UnwindSafe for BigFive2003Encoder","synthetic":true,"types":[]},{"text":"impl UnwindSafe for EncoderOnlyUTF8Encoding","synthetic":true,"types":[]}];
implementors["idna"] = [{"text":"impl UnwindSafe for Config","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Errors","synthetic":true,"types":[]}];
implementors["lexical_core"] = [{"text":"impl UnwindSafe for Error","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ErrorCode","synthetic":true,"types":[]}];
implementors["memchr"] = [{"text":"impl&lt;'a&gt; UnwindSafe for Memchr&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; UnwindSafe for Memchr2&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; UnwindSafe for Memchr3&lt;'a&gt;","synthetic":true,"types":[]}];
implementors["nom"] = [{"text":"impl UnwindSafe for CompareResult","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Needed","synthetic":true,"types":[]},{"text":"impl&lt;E&gt; UnwindSafe for Err&lt;E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I&gt; UnwindSafe for VerboseError&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl UnwindSafe for VerboseErrorKind","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ErrorKind","synthetic":true,"types":[]},{"text":"impl&lt;I, E, F&gt; UnwindSafe for ParserIterator&lt;I, E, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: UnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;F: UnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;I: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Endianness","synthetic":true,"types":[]}];
implementors["rustyknife"] = [{"text":"impl UnwindSafe for Legacy","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Intl","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ContentDisposition","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ContentTransferEncoding","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Param","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; UnwindSafe for Params&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Keyword","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Value","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Path","synthetic":true,"types":[]},{"text":"impl UnwindSafe for SMTPString","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ForwardPath","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ReversePath","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Command","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Mailbox","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Group","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Address","synthetic":true,"types":[]},{"text":"impl UnwindSafe for DSNMailParams","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Notify","synthetic":true,"types":[]},{"text":"impl UnwindSafe for DSNRet","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Domain","synthetic":true,"types":[]},{"text":"impl UnwindSafe for QuotedString","synthetic":true,"types":[]},{"text":"impl UnwindSafe for DotAtom","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Mailbox","synthetic":true,"types":[]},{"text":"impl UnwindSafe for LocalPart","synthetic":true,"types":[]},{"text":"impl UnwindSafe for DomainPart","synthetic":true,"types":[]},{"text":"impl UnwindSafe for AddressLiteral","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Param","synthetic":true,"types":[]}];
implementors["unicode_bidi"] = [{"text":"impl UnwindSafe for ParagraphInfo","synthetic":true,"types":[]},{"text":"impl&lt;'text&gt; UnwindSafe for InitialInfo&lt;'text&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'text&gt; UnwindSafe for BidiInfo&lt;'text&gt;","synthetic":true,"types":[]},{"text":"impl UnwindSafe for BidiClass","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Level","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Error","synthetic":true,"types":[]}];
implementors["unicode_normalization"] = [{"text":"impl&lt;I&gt; UnwindSafe for Decompositions&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I&gt; UnwindSafe for Recompositions&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;I&gt; UnwindSafe for StreamSafe&lt;I&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;I: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl UnwindSafe for IsNormalized","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()