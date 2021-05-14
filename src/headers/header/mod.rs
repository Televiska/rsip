pub use tokenizer::Tokenizer;

pub mod accept;
pub mod accept_encoding;
pub mod accept_language;
pub mod alert_info;
pub mod allow;
pub mod authentication_info;
pub mod authorization;
pub mod call_id;
pub mod call_info;
pub mod contact;
pub mod content_disposition;
pub mod content_encoding;
pub mod content_language;
pub mod content_length;
pub mod content_type;
pub mod cseq;
pub mod date;
pub mod error_info;
pub mod event;
pub mod expires;
pub mod from;
pub mod in_reply_to;
pub mod max_forwards;
pub mod mime_version;
pub mod min_expires;
pub mod organization;
pub mod priority;
pub mod proxy_authenticate;
pub mod proxy_authorization;
pub mod proxy_require;
pub mod record_route;
pub mod reply_to;
pub mod require;
pub mod retry_after;
pub mod route;
pub mod server;
pub mod subject;
pub mod subscription_state;
pub mod supported;
pub mod timestamp;
pub mod to;
pub mod unsupported;
pub mod user_agent;
pub mod via;
pub mod warning;
pub mod www_authenticate;

pub use accept::Accept;
pub use accept_encoding::AcceptEncoding;
pub use accept_language::AcceptLanguage;
pub use alert_info::AlertInfo;
pub use allow::Allow;
pub use authentication_info::AuthenticationInfo;
pub use authorization::Authorization;
pub use call_id::CallId;
pub use call_info::CallInfo;
pub use contact::Contact;
pub use content_disposition::ContentDisposition;
pub use content_encoding::ContentEncoding;
pub use content_language::ContentLanguage;
pub use content_length::ContentLength;
pub use content_type::ContentType;
pub use cseq::CSeq;
pub use date::Date;
pub use error_info::ErrorInfo;
pub use event::Event;
pub use expires::Expires;
pub use from::From;
pub use in_reply_to::InReplyTo;
pub use max_forwards::MaxForwards;
pub use mime_version::MimeVersion;
pub use min_expires::MinExpires;
pub use organization::Organization;
pub use priority::Priority;
pub use proxy_authenticate::ProxyAuthenticate;
pub use proxy_authorization::ProxyAuthorization;
pub use proxy_require::ProxyRequire;
pub use record_route::RecordRoute;
pub use reply_to::ReplyTo;
pub use require::Require;
pub use retry_after::RetryAfter;
pub use route::Route;
pub use server::Server;
pub use subject::Subject;
pub use subscription_state::SubscriptionState;
pub use supported::Supported;
pub use timestamp::Timestamp;
pub use to::To;
pub use unsupported::Unsupported;
pub use user_agent::UserAgent;
pub use via::Via;
pub use warning::Warning;
pub use www_authenticate::WwwAuthenticate;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Header {
    Accept(Accept),
    AcceptEncoding(AcceptEncoding),
    AcceptLanguage(AcceptLanguage),
    AlertInfo(AlertInfo),
    Allow(Allow),
    AuthenticationInfo(AuthenticationInfo),
    Authorization(Authorization),
    CSeq(CSeq),
    CallId(CallId),
    CallInfo(CallInfo),
    Contact(Contact),
    ContentDisposition(ContentDisposition),
    ContentEncoding(ContentEncoding),
    ContentLanguage(ContentLanguage),
    ContentLength(ContentLength),
    ContentType(ContentType),
    Date(Date),
    ErrorInfo(ErrorInfo),
    Event(Event),
    Expires(Expires),
    From(From),
    InReplyTo(InReplyTo),
    MaxForwards(MaxForwards),
    MimeVersion(MimeVersion),
    MinExpires(MinExpires),
    Organization(Organization),
    Other(String, String),
    Priority(Priority),
    ProxyAuthenticate(ProxyAuthenticate),
    ProxyAuthorization(ProxyAuthorization),
    ProxyRequire(ProxyRequire),
    RecordRoute(RecordRoute),
    ReplyTo(ReplyTo),
    Require(Require),
    RetryAfter(RetryAfter),
    Route(Route),
    Server(Server),
    Subject(Subject),
    SubscriptionState(SubscriptionState),
    Supported(Supported),
    Timestamp(Timestamp),
    To(To),
    Unsupported(Unsupported),
    UserAgent(UserAgent),
    Via(Via),
    Warning(Warning),
    WwwAuthenticate(WwwAuthenticate),
}

pub mod tokenizer {
    use super::*;
    use crate::{Error, NomError};
    use macros::Utf8Tokenizer;
    use std::convert::TryInto;

    impl<'a> TryInto<Header> for Tokenizer<'a> {
        type Error = Error;

        fn try_into(self) -> Result<Header, Error> {
            let tokenizer: Utf8Tokenizer = self.try_into()?;

            match tokenizer.name {
                s if s.eq_ignore_ascii_case("Accept") => {
                    Ok(Header::Accept(Accept::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Accept-Encoding") => {
                    Ok(Header::AcceptEncoding(AcceptEncoding::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Accept-Language") => {
                    Ok(Header::AcceptLanguage(AcceptLanguage::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Alert-Info") => {
                    Ok(Header::AlertInfo(AlertInfo::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Allow") => {
                    Ok(Header::Allow(Allow::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Authentication-Info") => Ok(
                    Header::AuthenticationInfo(AuthenticationInfo::new(tokenizer.value)),
                ),
                s if s.eq_ignore_ascii_case("Authorization") => {
                    Ok(Header::Authorization(Authorization::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("CSeq") => Ok(Header::CSeq(CSeq::new(tokenizer.value))),
                s if s.eq_ignore_ascii_case("Call-Id") => {
                    Ok(Header::CallId(CallId::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Call-Info") => {
                    Ok(Header::CallInfo(CallInfo::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Contact") => {
                    Ok(Header::Contact(Contact::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Content-Disposition") => Ok(
                    Header::ContentDisposition(ContentDisposition::new(tokenizer.value)),
                ),
                s if s.eq_ignore_ascii_case("Content-Encoding") => Ok(Header::ContentEncoding(
                    ContentEncoding::new(tokenizer.value),
                )),
                s if s.eq_ignore_ascii_case("Content-Language") => Ok(Header::ContentLanguage(
                    ContentLanguage::new(tokenizer.value),
                )),
                s if s.eq_ignore_ascii_case("Content-Length") => {
                    Ok(Header::ContentLength(ContentLength::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Content-Type") => {
                    Ok(Header::ContentType(ContentType::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Date") => Ok(Header::Date(Date::new(tokenizer.value))),
                s if s.eq_ignore_ascii_case("Error-Info") => {
                    Ok(Header::ErrorInfo(ErrorInfo::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Event") => {
                    Ok(Header::Event(Event::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Expires") => {
                    Ok(Header::Expires(Expires::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("From") => Ok(Header::From(From::new(tokenizer.value))),
                s if s.eq_ignore_ascii_case("In-Reply-To") => {
                    Ok(Header::InReplyTo(InReplyTo::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Max-Forwards") => {
                    Ok(Header::MaxForwards(MaxForwards::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Mime-Version") => {
                    Ok(Header::MimeVersion(MimeVersion::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Min-Expires") => {
                    Ok(Header::MinExpires(MinExpires::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Organization") => {
                    Ok(Header::Organization(Organization::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Priority") => {
                    Ok(Header::Priority(Priority::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Proxy-Authenticate") => Ok(Header::ProxyAuthenticate(
                    ProxyAuthenticate::new(tokenizer.value),
                )),
                s if s.eq_ignore_ascii_case("Proxy-Authorization") => Ok(
                    Header::ProxyAuthorization(ProxyAuthorization::new(tokenizer.value)),
                ),
                s if s.eq_ignore_ascii_case("Proxy-Require") => {
                    Ok(Header::ProxyRequire(ProxyRequire::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Record-Route") => {
                    Ok(Header::RecordRoute(RecordRoute::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Reply-To") => {
                    Ok(Header::ReplyTo(ReplyTo::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Require") => {
                    Ok(Header::Require(Require::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Retry-After") => {
                    Ok(Header::RetryAfter(RetryAfter::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Route") => {
                    Ok(Header::Route(Route::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Server") => {
                    Ok(Header::Server(Server::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Subject") => {
                    Ok(Header::Subject(Subject::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Supported") => {
                    Ok(Header::Supported(Supported::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Timestamp") => {
                    Ok(Header::Timestamp(Timestamp::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("To") => Ok(Header::To(To::new(tokenizer.value))),
                s if s.eq_ignore_ascii_case("Unsupported") => {
                    Ok(Header::Unsupported(Unsupported::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("User-Agent") => {
                    Ok(Header::UserAgent(UserAgent::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("Via") => Ok(Header::Via(Via::new(tokenizer.value))),
                s if s.eq_ignore_ascii_case("Warning") => {
                    Ok(Header::Warning(Warning::new(tokenizer.value)))
                }
                s if s.eq_ignore_ascii_case("WWW-Authenticate") => Ok(Header::WwwAuthenticate(
                    WwwAuthenticate::new(tokenizer.value),
                )),
                _ => Ok(Header::Other(tokenizer.name.into(), tokenizer.value.into())),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Utf8Tokenizer)]
    pub struct Tokenizer<'a> {
        pub name: &'a [u8],
        pub value: &'a [u8],
    }

    impl<'a> Tokenizer<'a> {
        pub fn tokenize(part: &'a [u8]) -> Result<(&'a [u8], Self), NomError<'a>> {
            use nom::{
                bytes::complete::{tag, take_until},
                character::complete::space0,
                sequence::tuple,
            };

            let (rem, (name, _, _, value, _)) = tuple((
                take_until(":"),
                tag(":"),
                space0,
                take_until("\r\n"),
                tag("\r\n"),
            ))(part)?;

            Ok((rem, (name, value).into()))
        }
    }

    impl<'a> std::convert::From<(&'a [u8], &'a [u8])> for Tokenizer<'a> {
        fn from(tuple: (&'a [u8], &'a [u8])) -> Self {
            Self {
                name: tuple.0,
                value: tuple.1,
            }
        }
    }
}
