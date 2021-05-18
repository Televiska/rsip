use rsip::headers::header::Accept;

#[test]
fn display() {
    assert_eq!(
        Accept::new("REGISTER, INVITE").to_string(),
        String::from("Accept: REGISTER, INVITE")
    );
}
