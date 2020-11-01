use crate::{Request, Response, SipMessage};

pub trait DebugExt {
    fn debug_headers(&self) -> String;
    fn debug(&self);
}

impl DebugExt for Request {
    fn debug_headers(&self) -> String {
        self.headers
            .iter()
            .map(|header| format!("{:?}", header))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn debug(&self) {
        println!(
            "Request: {} {} ({:?})\n{}\n{}\n",
            self.method,
            self.uri,
            self.version,
            self.debug_headers(),
            self.headers
        )
    }
}

impl DebugExt for Response {
    fn debug_headers(&self) -> String {
        self.headers
            .iter()
            .map(|header| format!("{:?}", header))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn debug(&self) {
        println!(
            "Request: {:?} ({:?})\n{}\n{}\n",
            self.code,
            self.version,
            self.debug_headers(),
            self.headers
        )
    }
}

impl DebugExt for SipMessage {
    fn debug_headers(&self) -> String {
        match self {
            Self::Request(request) => request.debug_headers(),
            Self::Response(response) => response.debug_headers(),
        }
    }

    fn debug(&self) {
        match self {
            Self::Request(request) => request.debug(),
            Self::Response(response) => response.debug(),
        };
    }
}
