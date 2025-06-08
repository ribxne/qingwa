use std::collections::HashMap;
use reqwest::{header::{ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT}};
use clap::{Parser};
use serde_json::{json};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    number: String,

    #[arg(short, long, default_value = "48")]
    country: String,
}



fn get_token_id(client: &reqwest::blocking::Client) -> String{
    let mut payload = HashMap::new();
    payload.insert("clientType", "CLIENT_TYPE_ANDROID");
    let res = client.post("https://www.googleapis.com/identitytoolkit/v3/relyingparty/signupNewUser?key=AIzaSyDe2Fgxn_8HJ6NrtJtp69YqXwocutAoa9Q")
        .json(&payload)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .unwrap();

    let debil =  res.json::<serde_json::Value>().unwrap();
    return debil["idToken"].as_str().unwrap().to_string();
}

fn send_verification_code(client: &reqwest::blocking::Client, id_token: &str, number: &str, country: &str){
    let payload = json!({
        "operationName": "SendVerificationCode",
        "query": "mutation SendVerificationCode($input: SendVerificationCodeInput!) { sendVerificationCode(input: $input) { retryAfterSeconds } }",
        "variables": {
            "input": {
                "phoneNumber": {
                    "countryCode": country.to_string(),
                    // "countryCode": &args.country,
                    "nationalNumber": number.to_string(),
                }
            }
        }
    });

    let headers = {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", id_token).parse().unwrap(),
        );
        headers
    };
    let res = client.post("https://super-account.spapp.zabka.pl/")
        .json(&payload)
        .headers(headers)
        .send()
        .unwrap();
    println!("{}", res.json::<serde_json::Value>().unwrap()); 
}

fn phone_auth(client: &reqwest::blocking::Client, id_token: &str, number: &str, country: &str, verification_code: &str) -> String{
    let headers = {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", id_token).parse().unwrap(),
        );
        headers.insert(
            USER_AGENT,
            "okhttp/4.12.0".parse().unwrap(),
        );
        headers.insert(
            "x-apollo-operation-id",
            "a531998ec966db0951239efb91519560346cfecac77459fe3b85c5b786fa41de".parse().unwrap(),
        );
        headers.insert(
            "x-apollo-operation-name",
            "SignInWithPhone".parse().unwrap(),
        );
        headers.insert(
            ACCEPT,
            "multipart/mixed; deferSpec=20220824, application/json".parse().unwrap(),
        );
        headers.insert(
            CONTENT_LENGTH,
            "250".parse().unwrap(),
        );
        headers
    };
    
    let payload = json!(
         {
            "operationName": "SignInWithPhone",
            "variables": {
                "input": {
                    "phoneNumber": {
                        "countryCode": country, 
                        "nationalNumber": number,
                    },
                    "verificationCode": verification_code,
                }
            },
            "query": "mutation SignInWithPhone($input: SignInInput!) { signIn(input: $input) { customToken } }"
        }
    );
    
    let res = client.post("https://super-account.spapp.zabka.pl/")
        .json(&payload)
        .headers(headers)
        .send()
        .unwrap();

    let debil =  res.json::<serde_json::Value>().unwrap();
    return debil["data"]["signIn"]["customToken"].as_str().unwrap().to_string();
}

fn verify_custom_token(client: &reqwest::blocking::Client, custom_token: &str) -> String{

    let payload = json!({
        "token": custom_token,
        "returnSecureToken": "True",
    });

    let res = client.post("https://www.googleapis.com/identitytoolkit/v3/relyingparty/verifyCustomToken?key=AIzaSyDe2Fgxn_8HJ6NrtJtp69YqXwocutAoa9Q")
        .json(&payload)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .unwrap();

    let debil =  res.json::<serde_json::Value>().unwrap();
    
    return debil["idToken"].as_str().unwrap().to_string();
}

fn get_user_secrets(client: &reqwest::blocking::Client, custom_token: &str) -> serde_json::Value {
    let payload = json!({
            "operationName": "QrCode",
            "query": "
                query QrCode { 
                    qrCode { 
                        loyalSecret 
                        paySecret 
                        ployId 
                    } 
                }
            ",
            "variables": {}
        });

    let headers = {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Cache-Control",
            "no-cache".parse().unwrap(),
        );
        headers.insert(
            CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers.insert(
            USER_AGENT,
            "Zappka/40038 (Horizon; nintendo/ctr; 56c41945-ba88-4543-a525-4e8f7d4a5812) REL/28".parse().unwrap(),
        );
        headers.insert(
            ACCEPT,
            "application/json".parse().unwrap(),
        );
        headers.insert(
            AUTHORIZATION,
            custom_token.parse().unwrap(),
        );
        headers
    };

    let res = client.post("https://api.spapp.zabka.pl/")
        .json(&payload)
        .headers(headers)
        .send()
        .unwrap();
    return res.json::<serde_json::Value>().unwrap()["data"]["qrCode"].clone();
    // return (data["loyalSecret"].as_str().unwrap().to_string(), data["paySecret"].as_str().unwrap().to_string(), data["ployId"].as_str().unwrap().to_string())
}

fn get_nano_status(client: &reqwest::blocking::Client, custom_token: &str) -> bool{

    let payload = json!({
        "operationName": "PaymentCards",
        "query": "query PaymentCards { paymentCards { paymentCards { id isDefault lastFourDigits } } }",
        "variables": {}
    });

    let headers = {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Cache-Control",
            "no-cache".parse().unwrap(),
        );
        headers.insert(
            CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers.insert(
            USER_AGENT,
            "Zappka/40038 (Horizon; nintendo/ctr; 56c41945-ba88-4543-a525-4e8f7d4a5812) REL/28".parse().unwrap(),
        );
        headers.insert(
            ACCEPT,
            "application/json".parse().unwrap(),
        );
        headers.insert(
            AUTHORIZATION,
            custom_token.parse().unwrap(),
        );
        headers
    };

    let res = client.post("https://api.spapp.zabka.pl/")
        .json(&payload)
        .headers(headers)
        .send()
        .unwrap();

    let cards = &res.json::<serde_json::Value>().unwrap()["data"]["paymentCards"]["paymentCards"];

    if cards.is_array() && cards.as_array().unwrap().is_empty(){
        return true
    } else {
        return false
    }
}

// fn main() {
//     let args = Args::parse();
//     let client = reqwest::blocking::Client::new();

//     let id_token = get_token_id(&client);
//     send_verification_code(&client, id_token.as_str(), &args.number, &args.country);

//     use std::io;
//     let mut verification_code = String::new();
//     io::stdin().read_line(&mut verification_code).unwrap();
//     let custom_token = phone_auth(&client, id_token.as_str(), &args.number, &args.country, verification_code.trim_end());
//     let identity_provider_token = verify_custom_token(&client, custom_token.as_str());
//     println!("{}", identity_provider_token);
//     println!("{}", get_user_secrets(&client, identity_provider_token.as_str()));
//     println!("{}", get_nano_status(&client, identity_provider_token.as_str()));
// }