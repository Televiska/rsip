use crate::{
    headers::{self, Header},
    Error, Request, Response, SipMessage,
};

pub trait HeadersExt {
    fn to_header(&self) -> Result<&headers::To, Error>;
    fn to_header_mut(&mut self) -> Result<&mut headers::To, Error>;

    fn from_header(&self) -> Result<&headers::From, Error>;
    fn from_header_mut(&mut self) -> Result<&mut headers::From, Error>;

    fn via_header(&self) -> Result<&headers::Via, Error>;
    fn via_header_mut(&mut self) -> Result<&mut headers::Via, Error>;

    fn call_id_header(&self) -> Result<&headers::CallId, Error>;
    fn call_id_header_mut(&mut self) -> Result<&mut headers::CallId, Error>;

    fn cseq_header(&self) -> Result<&headers::CSeq, Error>;
    fn cseq_header_mut(&mut self) -> Result<&mut headers::CSeq, Error>;

    fn max_forwards_header(&self) -> Result<&headers::MaxForwards, Error>;
    fn max_forwards_header_mut(&mut self) -> Result<&mut headers::MaxForwards, Error>;

    fn contact_header(&self) -> Result<&headers::Contact, Error>;
    fn contact_header_mut(&mut self) -> Result<&mut headers::Contact, Error>;

    /*
        fn from_header_tag(&self) -> Result<&String, SipMessageError>;
        fn to_header_tag(&self) -> Result<&String, SipMessageError>;

        fn set_from_header_tag(&mut self, tag: String);

        fn from_header_username(&self) -> Result<&String, SipMessageError>;

        fn set_to_header_tag(&mut self, tag: String);

        fn to_header_username(&self) -> Result<&String, SipMessageError>;


        fn via_header_branch(&self) -> Result<&String, SipMessageError>;

        fn contact_header_username(&self) -> Result<&String, SipMessageError>;

        /// Returns number of seconds if it's specified in the Contact header
        fn contact_header_expires(&self) -> Result<u32, SipMessageError>;

        fn expires_header(&self) -> Result<u32, SipMessageError>;

        fn expires_header_mut(&mut self) -> Result<&mut u32, SipMessageError>;
    */
}

impl HeadersExt for Request {
    fn to_header(&self) -> Result<&headers::To, Error> {
        header!(self.headers().iter(), Header::To, Error::MissingHeader)
    }
    fn to_header_mut(&mut self) -> Result<&mut headers::To, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::To,
            Error::MissingHeader
        )
    }

    fn from_header(&self) -> Result<&headers::From, Error> {
        header!(self.headers().iter(), Header::From, Error::MissingHeader)
    }
    fn from_header_mut(&mut self) -> Result<&mut headers::From, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::From,
            Error::MissingHeader
        )
    }

    fn via_header(&self) -> Result<&headers::Via, Error> {
        header!(self.headers().iter(), Header::Via, Error::MissingHeader)
    }
    fn via_header_mut(&mut self) -> Result<&mut headers::Via, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::Via,
            Error::MissingHeader
        )
    }

    fn call_id_header(&self) -> Result<&headers::CallId, Error> {
        header!(self.headers().iter(), Header::CallId, Error::MissingHeader)
    }
    fn call_id_header_mut(&mut self) -> Result<&mut headers::CallId, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::CallId,
            Error::MissingHeader
        )
    }

    fn cseq_header(&self) -> Result<&headers::CSeq, Error> {
        header!(self.headers().iter(), Header::CSeq, Error::MissingHeader)
    }
    fn cseq_header_mut(&mut self) -> Result<&mut headers::CSeq, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::CSeq,
            Error::MissingHeader
        )
    }

    fn max_forwards_header(&self) -> Result<&headers::MaxForwards, Error> {
        header!(self.headers().iter(), Header::MaxForwards, Error::MissingHeader)
    }
    fn max_forwards_header_mut(&mut self) -> Result<&mut headers::MaxForwards, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::MaxForwards,
            Error::MissingHeader
        )
    }

    fn contact_header(&self) -> Result<&headers::Contact, Error> {
        header!(self.headers().iter(), Header::Contact, Error::MissingHeader)
    }
    fn contact_header_mut(&mut self) -> Result<&mut headers::Contact, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::Contact,
            Error::MissingHeader
        )
    }
}

impl HeadersExt for Response {
    fn to_header(&self) -> Result<&headers::To, Error> {
        header!(self.headers().iter(), Header::To, Error::MissingHeader)
    }
    fn to_header_mut(&mut self) -> Result<&mut headers::To, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::To,
            Error::MissingHeader
        )
    }

    fn from_header(&self) -> Result<&headers::From, Error> {
        header!(self.headers().iter(), Header::From, Error::MissingHeader)
    }
    fn from_header_mut(&mut self) -> Result<&mut headers::From, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::From,
            Error::MissingHeader
        )
    }

    fn via_header(&self) -> Result<&headers::Via, Error> {
        header!(self.headers().iter(), Header::Via, Error::MissingHeader)
    }
    fn via_header_mut(&mut self) -> Result<&mut headers::Via, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::Via,
            Error::MissingHeader
        )
    }

    fn call_id_header(&self) -> Result<&headers::CallId, Error> {
        header!(self.headers().iter(), Header::CallId, Error::MissingHeader)
    }
    fn call_id_header_mut(&mut self) -> Result<&mut headers::CallId, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::CallId,
            Error::MissingHeader
        )
    }

    fn cseq_header(&self) -> Result<&headers::CSeq, Error> {
        header!(self.headers().iter(), Header::CSeq, Error::MissingHeader)
    }
    fn cseq_header_mut(&mut self) -> Result<&mut headers::CSeq, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::CSeq,
            Error::MissingHeader
        )
    }

    fn max_forwards_header(&self) -> Result<&headers::MaxForwards, Error> {
        header!(self.headers().iter(), Header::MaxForwards, Error::MissingHeader)
    }
    fn max_forwards_header_mut(&mut self) -> Result<&mut headers::MaxForwards, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::MaxForwards,
            Error::MissingHeader
        )
    }

    fn contact_header(&self) -> Result<&headers::Contact, Error> {
        header!(self.headers().iter(), Header::Contact, Error::MissingHeader)
    }
    fn contact_header_mut(&mut self) -> Result<&mut headers::Contact, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::Contact,
            Error::MissingHeader
        )
    }
}

impl HeadersExt for SipMessage {
    fn to_header(&self) -> Result<&headers::To, Error> {
        header!(self.headers().iter(), Header::To, Error::MissingHeader)
    }
    fn to_header_mut(&mut self) -> Result<&mut headers::To, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::To,
            Error::MissingHeader
        )
    }

    fn from_header(&self) -> Result<&headers::From, Error> {
        header!(self.headers().iter(), Header::From, Error::MissingHeader)
    }
    fn from_header_mut(&mut self) -> Result<&mut headers::From, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::From,
            Error::MissingHeader
        )
    }

    fn via_header(&self) -> Result<&headers::Via, Error> {
        header!(self.headers().iter(), Header::Via, Error::MissingHeader)
    }
    fn via_header_mut(&mut self) -> Result<&mut headers::Via, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::Via,
            Error::MissingHeader
        )
    }

    fn call_id_header(&self) -> Result<&headers::CallId, Error> {
        header!(self.headers().iter(), Header::CallId, Error::MissingHeader)
    }
    fn call_id_header_mut(&mut self) -> Result<&mut headers::CallId, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::CallId,
            Error::MissingHeader
        )
    }

    fn cseq_header(&self) -> Result<&headers::CSeq, Error> {
        header!(self.headers().iter(), Header::CSeq, Error::MissingHeader)
    }
    fn cseq_header_mut(&mut self) -> Result<&mut headers::CSeq, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::CSeq,
            Error::MissingHeader
        )
    }

    fn max_forwards_header(&self) -> Result<&headers::MaxForwards, Error> {
        header!(self.headers().iter(), Header::MaxForwards, Error::MissingHeader)
    }
    fn max_forwards_header_mut(&mut self) -> Result<&mut headers::MaxForwards, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::MaxForwards,
            Error::MissingHeader
        )
    }

    fn contact_header(&self) -> Result<&headers::Contact, Error> {
        header!(self.headers().iter(), Header::Contact, Error::MissingHeader)
    }
    fn contact_header_mut(&mut self) -> Result<&mut headers::Contact, Error> {
        header!(
            self.headers_mut().iter_mut(),
            Header::Contact,
            Error::MissingHeader
        )
    }
}
