// use qrcode::render::unicode;
// use qrcode::{EcLevel, QrCode};
use clap::{Parser, Subcommand};
use tokio::io::AsyncBufReadExt;

mod autofill;
mod login;
// mod totp;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<SubCmd>,
}

#[derive(Subcommand, Debug)]
enum SubCmd {
    Login {
        #[arg(short, long)]
        number: String,

        #[arg(short, long, default_value = "48")]
        country: String,
    },
}

// fn make_qr(totp: u32, userid: &str) -> Result<(), &'static str> {
//     // todo: add loyal support and detection
//     let url = format!("https://srln.pl/view/dashboard?ploy={userid}&pay={totp:06}");

//     let code = QrCode::with_error_correction_level(url.as_bytes(), EcLevel::Q)
//         .map_err(|_| "QR gen failed")?;

//     let string = code
//         .render::<unicode::Dense1x2>()
//         .quiet_zone(false)
//         .module_dimensions(1, 1)
//         .build();

//     println!("{string}");

//     Ok(())
// }

async fn login(number: String, country: String) {
    let client = reqwest::Client::new();

    let id_token = login::get_token_id(&client).await.unwrap();
    login::send_verification_code(&client, &id_token, &number, &country).await;
    let verification_code = get_sms_code_from_dbus_or_user().await;

    let custom_token = login::phone_auth(
        &client,
        &id_token,
        &number,
        &country,
        verification_code.trim_end(),
    )
    .await
    .unwrap();
    let identity_provider_token = login::verify_custom_token(&client, custom_token.as_str())
        .await
        .unwrap();
    // println!("{}", identity_provider_token);
    let credentials = login::get_user_secrets(&client, identity_provider_token.as_str())
        .await
        .unwrap();
    let nano_status = login::get_nano_status(&client, identity_provider_token.as_str())
        .await
        .unwrap();
    println!("Please add the following values to the environment (.bashrc, etc.)");
    println!(
        "PAY_SECRET = {}
LOYAL_SECRET = {}
PLOY_ID = {}",
        credentials["paySecret"], credentials["loyalSecret"], credentials["ployId"]
    );
    if nano_status {
        println!("NANO_ENABLED = \"TRUE\"");
    }
    // println!("{}", login::get_user_secrets(&client, identity_provider_token.as_str()).await.unwrap());
    // println!("{}", login::get_nano_status(&client, identity_provider_token.as_str()).await.unwrap());
}

async fn input() -> String {
    let mut verification_code = String::new();
    tokio::io::BufReader::new(tokio::io::stdin())
        .read_line(&mut verification_code)
        .await
        .unwrap();
    return verification_code.trim_end().to_string();
}

async fn get_sms_code_from_dbus_or_user() -> String {
    tokio::select! {
        code = input() => {return code},
        result = autofill::wait_for_sms_code() => {
            println!("Autofilled code from sms");
            let code = result.unwrap();
            println!("sms code: {code}");
            return code
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Args::parse();
    match args.command {
        Some(SubCmd::Login { number, country }) => login(number, country).await,
        None => todo!(),
    }
    // let secret_hex = ""; // paySecret
    // let userid = ""; // your żappka user id/ployId
    // loop {
    //     clearscreen::clear().ok();
    //     make_qr(calculate_totp(secret_hex), userid).unwrap();
    //     println!("{}", calculate_totp(secret_hex));
    //     thread::sleep(Duration::from_secs(2));
    // }
}
