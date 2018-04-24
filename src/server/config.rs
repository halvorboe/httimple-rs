use rustls;
use fs;
use std::io::{Write, Read, BufReader};
use std::sync::Arc;

fn load_certs(filename: &str) -> Vec<rustls::Certificate> {
    let certfile = fs::File::open(filename).expect("cannot open certificate file");
    let mut reader = BufReader::new(certfile);
    rustls::internal::pemfile::certs(&mut reader).unwrap()
}

fn load_private_key(filename: &str) -> rustls::PrivateKey {
    let rsa_keys = {
        let keyfile = fs::File::open(filename)
            .expect("cannot open private key file");
        let mut reader = BufReader::new(keyfile);
        rustls::internal::pemfile::rsa_private_keys(&mut reader)
            .expect("file contains invalid rsa private key")
    };

    let pkcs8_keys = {
        let keyfile = fs::File::open(filename)
            .expect("cannot open private key file");
        let mut reader = BufReader::new(keyfile);
        rustls::internal::pemfile::pkcs8_private_keys(&mut reader)
            .expect("file contains invalid pkcs8 private key (encrypted keys not supported)")
    };

    // prefer to load pkcs8 keys
    if !pkcs8_keys.is_empty() {
        pkcs8_keys[0].clone()
    } else {
        assert!(!rsa_keys.is_empty());
        rsa_keys[0].clone()
    }
}

fn load_ocsp(filename: &Option<String>) -> Vec<u8> {
    let mut ret = Vec::new();

    if let &Some(ref name) = filename {
        fs::File::open(name)
            .expect("cannot open ocsp file")
            .read_to_end(&mut ret)
            .unwrap();
    }

    ret
}

pub fn make_config() -> Arc<rustls::ServerConfig> {

    let mut config = rustls::ServerConfig::new(rustls::NoClientAuth::new());

    let certs = load_certs("ca/rsa/end.fullchain");
    let privkey = load_private_key("ca/rsa/end.rsa");
    let ocsp = load_ocsp(&None);
    config.set_single_cert_with_ocsp_and_sct(certs, privkey, ocsp, vec![]);


    config.set_persistence(rustls::ServerSessionMemoryCache::new(256));


    config.set_protocols(&vec![String::from("h2")]);

    Arc::new(config)
}