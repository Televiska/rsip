use rsip::{
    common::{
        auth::{Algorithm, Qop},
        uri::{Host, HostWithPort},
        Method,
    },
    services::auth::DigestGenerator,
};

#[test]
fn digest() {
    let uri: HostWithPort = Host::from("/dir/index.html").into();

    let generator = DigestGenerator {
        username: "Mufasa",
        password: "Circle Of Life",
        algorithm: Algorithm::Md5,
        nonce: "dcd98b7102dd2f0e8b11d0f600bfb0c093",
        cnonce: "0a4f113b",
        nc: 1,
        method: Method::Register,
        qop: Some(Qop::Auth),
        uri: &uri.into(),
        realm: "testrealm@host.com",
    };

    assert_eq!(
        "59d17b90f0e821045ecceb843e5b38c4",
        generator.compute()
    );
}
