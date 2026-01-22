use totp_rs::{Algorithm, TOTP};

fn main() {
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        "hahaha".as_bytes().to_vec(),
        Some("Github".to_string()),
        "hzw-the-programmer@github.com".to_string(),
    )
    .unwrap();

    let url = totp.get_url();
    println!("{}", url);
    let code = totp.get_qr_base64().unwrap();
    println!("{}", code);
}
