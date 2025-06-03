use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::thread;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use hex::FromHex;
use qrcode::{QrCode, EcLevel};
use qrcode::render::unicode;

type HmacSha1 = Hmac<Sha1>;

const JAVA_INT_MAX: u32 = 2_147_483_647;

fn c(arr: &[u8], index: usize) -> u32 {
    let mut result: u32 = 0;
    for i in index..index + 4 {
        result = (result << 8) | (arr[i] as u32 & 0xFF);
    }
    result
}

fn calculate_totp(secret_hex: &str) -> u32 {

    let secret = <[u8; 32]>::from_hex(secret_hex).expect("Invalid hex string");

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() / 30;

    let msg = ts.to_be_bytes();

    let mut mac = HmacSha1::new_from_slice(&secret).expect("HMAC can take key of any size");
    mac.update(&msg);
    let output_bytes = mac.finalize().into_bytes();

    let offset = (output_bytes[output_bytes.len() - 1] & 0x0f) as usize;
    let magic_number = (c(&output_bytes, offset) & JAVA_INT_MAX) % 1_000_000;

    return magic_number
}

fn make_qr(totp: u32, userid: &str) -> Result<(), &'static str> {
    // todo: add loyal support and detection
    let url = format!(
        "https://srln.pl/view/dashboard?ploy={}&pay={:06}",
        userid, totp
    );

    let code = QrCode::with_error_correction_level(url.as_bytes(), EcLevel::Q)
        .map_err(|_| "QR gen failed")?;

    let string = code
        .render::<unicode::Dense1x2>()
        .quiet_zone(false)
        .module_dimensions(1, 1)
        .build();

    println!("{}", string);

    Ok(())
}

fn main(){
    let secret_hex = ""; // paySecret
    let userid = ""; // your żappka user id/ployId
    loop{
        clearscreen::clear().ok();
        make_qr(calculate_totp(secret_hex), userid);
        println!("{}", calculate_totp(secret_hex));
        thread::sleep(Duration::from_secs(2));
    }
}