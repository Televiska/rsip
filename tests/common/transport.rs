use rsip::common::transport::{Tokenizer, Transport};
use std::convert::TryInto;

#[test]
fn display() {
    assert_eq!(Transport::Udp.to_string(), String::from("UDP"));

    assert_eq!(Transport::Tcp.to_string(), String::from("TCP"));
}

#[test]
fn parser() {
    assert_eq!(
        Tokenizer::from("UDP".as_bytes()).try_into(),
        Ok(Transport::Udp)
    );

    assert_eq!(
        Tokenizer::from("TCP".as_bytes()).try_into(),
        Ok(Transport::Tcp)
    );
}

#[test]
fn tokenizer() {
    assert_eq!(
        Tokenizer::tokenize(b"UDP "),
        Ok((" ".as_bytes(), "UDP".as_bytes().into())),
    );

    assert_eq!(
        Tokenizer::tokenize(b"TCP"),
        Ok(("".as_bytes(), "TCP".as_bytes().into())),
    );
}
