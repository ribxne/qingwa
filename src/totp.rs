use hex::FromHex;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

type HmacSha1 = Hmac<Sha1>;

const JAVA_INT_MAX: u32 = 2_147_483_647;

fn c(arr: &[u8], index: usize) -> u32 {
    let mut result: u32 = 0;
    for byte in arr.iter().skip(index).take(4) {
        result = (result << 8) | (u32::from(*byte) & 0xFF);
    }
    result
}

fn calculate_totp(secret_hex: &str) -> u32 {
    let secret = <[u8; 32]>::from_hex(secret_hex).expect("Invalid hex string");

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        / 30;

    let msg = ts.to_be_bytes();

    let mut mac = HmacSha1::new_from_slice(&secret).expect("HMAC can take key of any size");
    mac.update(&msg);
    let output_bytes = mac.finalize().into_bytes();

    let offset = (output_bytes[output_bytes.len() - 1] & 0x0f) as usize;
    let magic_number = (c(&output_bytes, offset) & JAVA_INT_MAX) % 1_000_000;

    return magic_number;
}