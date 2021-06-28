use crate::{
    error::Header as ErrorHeader,
    headers::{self, Header},
    Error,
};

pub trait HeadersExt: super::HasHeaders {
    fn to_header(&self) -> Result<&headers::To, Error> {
        header!(
            self.headers().iter(),
            Header::To,
            Error::MissingHeader(ErrorHeader::To)
        )
    }

    #[allow(clippy::wrong_self_convention)]
    fn to_header_mut(&mut self) -> Result<&mut headers::To, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::To,
            Error::MissingHeader(ErrorHeader::To)
        )
    }

    fn from_header(&self) -> Result<&headers::From, Error> {
        header!(
            self.headers().iter(),
            Header::From,
            Error::MissingHeader(ErrorHeader::From)
        )
    }
    fn from_header_mut(&mut self) -> Result<&mut headers::From, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::From,
            Error::MissingHeader(ErrorHeader::From)
        )
    }

    fn via_header(&self) -> Result<&headers::Via, Error> {
        header!(
            self.headers().iter(),
            Header::Via,
            Error::MissingHeader(ErrorHeader::Via)
        )
    }
    fn via_header_mut(&mut self) -> Result<&mut headers::Via, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::Via,
            Error::MissingHeader(ErrorHeader::Via)
        )
    }

    fn call_id_header(&self) -> Result<&headers::CallId, Error> {
        header!(
            self.headers().iter(),
            Header::CallId,
            Error::MissingHeader(ErrorHeader::CallId)
        )
    }
    fn call_id_header_mut(&mut self) -> Result<&mut headers::CallId, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::CallId,
            Error::MissingHeader(ErrorHeader::CallId)
        )
    }
/*
    fn cseq_header(&self) -> Result<&headers::CSeq, Error> {
        header!(
            self.headers().iter(),
            Header::CSeq,
            Error::MissingHeader(ErrorHeader::CSeq)
        )
    }
    fn cseq_header_mut(&mut self) -> Result<&mut headers::CSeq, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::CSeq,
            Error::MissingHeader(ErrorHeader::CSeq)
        )
    }
*/

    fn max_forwards_header(&self) -> Result<&headers::MaxForwards, Error> {
        header!(
            self.headers().iter(),
            Header::MaxForwards,
            Error::MissingHeader(ErrorHeader::MaxForwards)
        )
    }
    fn max_forwards_header_mut(&mut self) -> Result<&mut headers::MaxForwards, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::MaxForwards,
            Error::MissingHeader(ErrorHeader::MaxForwards)
        )
    }

    fn contact_header(&self) -> Result<&headers::Contact, Error> {
        header!(
            self.headers().iter(),
            Header::Contact,
            Error::MissingHeader(ErrorHeader::Contact)
        )
    }
    fn contact_header_mut(&mut self) -> Result<&mut headers::Contact, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::Contact,
            Error::MissingHeader(ErrorHeader::Contact)
        )
    }
    fn contact_headers(&self) -> Vec<&headers::Contact> {
        all_headers!(self.headers().iter(), Header::Contact)
    }

    fn user_agent_header(&self) -> Result<&headers::UserAgent, Error> {
        header!(
            self.headers().iter(),
            Header::UserAgent,
            Error::MissingHeader(ErrorHeader::UserAgent)
        )
    }

    fn authorization_header(&self) -> Option<&headers::Authorization> {
        header_opt!(self.headers().iter(), Header::Authorization)
    }

    fn expires_header(&self) -> Option<&headers::Expires> {
        header_opt!(self.headers().iter(), Header::Expires)
    }

    fn min_expires_header(&self) -> Result<&headers::MinExpires, Error> {
        header!(
            self.headers().iter(),
            Header::MinExpires,
            Error::MissingHeader(ErrorHeader::MinExpires)
        )
    }

    fn transaction_id(&self) -> Result<String, Error> {
        use crate::headers::header::UntypedHeader;

        Ok(format!(
            "{}",
            self.via_header()?
                .clone()
                .typed()?
                .branch()
                .ok_or_else(|| Error::Unexpected("missing branch in via header!".into()))?
        ))
    }

    fn dialog_id(&self) -> Result<Option<String>, Error> {
        //use crate::common::uri::param::Tag;
        /*
                let (call_id, from_tag, to_tag): (Option<String>, Option<Tag>, Option<Tag>) = (
                    self.call_id_header().ok().cloned().map(Into::into),
                    self.from_header()
                        .ok()
                        .cloned()
                        .map(|h| h.typed().map(|h| h.tag().cloned()))
                        .transpose()?
                        .flatten(),
                    self.to_header()
                        .ok()
                        .cloned()
                        .map(|h| h.typed().map(|h| h.tag().cloned()))
                        .transpose()?
                        .flatten(),
                );

                Ok(match (call_id, from_tag, to_tag) {
                    (Some(call_id), Some(from), Some(to)) => Some(format!("{}-{}-{}", call_id, from, to)),
                    _ => None,
                })
        */
        Ok(None)
    }
}

impl HeadersExt for crate::Request {}
impl HeadersExt for crate::Response {}
impl HeadersExt for crate::SipMessage {}
