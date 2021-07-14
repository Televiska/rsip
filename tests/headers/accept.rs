use rsip::headers::*;

validate_untyped_header_trait!(Accept);

#[test]
fn display() {
    assert_eq!(
        Accept::new("REGISTER, INVITE").to_string(),
        String::from("Accept: REGISTER, INVITE")
    );
}
