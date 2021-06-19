pub struct HasHeadersImpl(pub rsip::headers::Headers);

impl rsip::message::HasHeaders for HasHeadersImpl {
    fn headers(&self) -> &rsip::headers::Headers {
        &self.0
    }

    fn headers_mut(&mut self) -> &mut rsip::headers::Headers {
        &mut self.0
    }
}

impl rsip::message::HeadersExt for HasHeadersImpl {}
