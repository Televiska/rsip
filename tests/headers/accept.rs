use rsip::headers::header::Accept;
use rsip::headers::header::UntypedHeader;

#[test]
fn display() {
    assert_eq!(
        Accept::new("REGISTER, INVITE").to_string(),
        String::from("Accept: REGISTER, INVITE")
    );
}
