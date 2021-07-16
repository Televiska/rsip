use rsip::{
    common::{
        uri::{Host, HostWithPort},
        Method,
    },
    headers::auth::{Algorithm, AuthQop},
    services::auth::DigestGenerator,
};

#[test]
fn digest() {
    let uri: HostWithPort = Host::from("/dir/index.html").into();

    let auth_qop = AuthQop::Auth {
        cnonce: "0a4f113b".into(),
        nc: 1,
    };

    let generator = DigestGenerator {
        username: "Mufasa",
        password: "Circle Of Life",
        algorithm: Algorithm::Md5,
        nonce: "dcd98b7102dd2f0e8b11d0f600bfb0c093",
        method: &Method::Register,
        qop: Some(&auth_qop),
        uri: &uri.into(),
        realm: "testrealm@host.com",
    };

    assert_eq!("59d17b90f0e821045ecceb843e5b38c4", generator.compute());
    assert_eq!(generator.verify("59d17b90f0e821045ecceb843e5b38c4"), true);
}
