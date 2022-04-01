use crate::{
    headers::{self, Header},
    param::Branch,
    Error,
};

/// Helpful trait to access most common headers from a [Request](crate::Request),
/// [Response](crate::Response) and [SipMessage](crate::SipMessage) structs.
///
/// Some headers that are expected to be there return a `Result<T, rsip::Error>` while others
/// return just an `Option<T>`, where `T` is the actual [untyped](crate::headers::untyped) header.
pub trait HeadersExt: super::HasHeaders {
    fn to_header(&self) -> Result<&headers::To, Error> {
        header!(
            self.headers().iter(),
            Header::To,
            Error::missing_header("To")
        )
    }

    fn to_header_mut(&mut self) -> Result<&mut headers::To, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::To,
            Error::missing_header("To")
        )
    }

    fn from_header(&self) -> Result<&headers::From, Error> {
        header!(
            self.headers().iter(),
            Header::From,
            Error::missing_header("From")
        )
    }
    fn from_header_mut(&mut self) -> Result<&mut headers::From, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::From,
            Error::missing_header("From")
        )
    }

    fn via_header(&self) -> Result<&headers::Via, Error> {
        header!(
            self.headers().iter(),
            Header::Via,
            Error::missing_header("Via")
        )
    }
    fn via_header_mut(&mut self) -> Result<&mut headers::Via, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::Via,
            Error::missing_header("Via")
        )
    }

    fn call_id_header(&self) -> Result<&headers::CallId, Error> {
        header!(
            self.headers().iter(),
            Header::CallId,
            Error::missing_header("CallID")
        )
    }
    fn call_id_header_mut(&mut self) -> Result<&mut headers::CallId, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::CallId,
            Error::missing_header("CallID")
        )
    }

    fn cseq_header(&self) -> Result<&headers::CSeq, Error> {
        header!(
            self.headers().iter(),
            Header::CSeq,
            Error::missing_header("CSeq")
        )
    }
    fn cseq_header_mut(&mut self) -> Result<&mut headers::CSeq, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::CSeq,
            Error::missing_header("CSeq")
        )
    }

    fn max_forwards_header(&self) -> Result<&headers::MaxForwards, Error> {
        header!(
            self.headers().iter(),
            Header::MaxForwards,
            Error::missing_header("Max-Forwards")
        )
    }
    fn max_forwards_header_mut(&mut self) -> Result<&mut headers::MaxForwards, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::MaxForwards,
            Error::missing_header("Max-Forwards")
        )
    }

    fn contact_header(&self) -> Result<&headers::Contact, Error> {
        header!(
            self.headers().iter(),
            Header::Contact,
            Error::missing_header("Contact")
        )
    }

    fn contact_header_mut(&mut self) -> Result<&mut headers::Contact, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::Contact,
            Error::missing_header("Contact")
        )
    }

    fn contact_headers(&self) -> Vec<&headers::Contact> {
        all_headers!(self.headers().iter(), Header::Contact)
    }

    fn record_route_header(&self) -> Option<&headers::RecordRoute> {
        header_opt!(self.headers().iter(), Header::RecordRoute)
    }

    fn route_header(&self) -> Option<&headers::Route> {
        header_opt!(self.headers().iter(), Header::Route)
    }

    fn user_agent_header(&self) -> Option<&headers::UserAgent> {
        header_opt!(self.headers().iter(), Header::UserAgent)
    }

    fn authorization_header(&self) -> Option<&headers::Authorization> {
        header_opt!(self.headers().iter(), Header::Authorization)
    }

    fn www_authenticate_header(&self) -> Option<&headers::WwwAuthenticate> {
        header_opt!(self.headers().iter(), Header::WwwAuthenticate)
    }

    fn expires_header(&self) -> Option<&headers::Expires> {
        header_opt!(self.headers().iter(), Header::Expires)
    }

    fn min_expires_header(&self) -> Option<&headers::MinExpires> {
        header_opt!(self.headers().iter(), Header::MinExpires)
    }

    fn transaction_id(&self) -> Result<Option<Branch>, Error> {
        use crate::headers::ToTypedHeader;

        Ok(self.via_header()?.clone().typed()?.branch().cloned())
    }
}

impl HeadersExt for crate::Request {}
impl HeadersExt for crate::Response {}
impl HeadersExt for crate::SipMessage {}
