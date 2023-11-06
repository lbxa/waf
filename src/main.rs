use waf::sqli;

fn main() {
    let (is_sqli, fingerprint) = sqli("' OR '1'='1' --").unwrap();
    assert!(is_sqli);
    assert_eq!("s&sos", fingerprint);
}
