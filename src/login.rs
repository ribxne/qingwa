use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT};
use serde_json::json;
use std::collections::HashMap;

pub async fn get_token_id(client: &reqwest::Client) -> Result<String, reqwest::Error> {
    let mut payload = HashMap::new();
    payload.insert("clientType", "CLIENT_TYPE_ANDROID");
    let res = client.post("https://www.googleapis.com/identitytoolkit/v3/relyingparty/signupNewUser?key=AIzaSyDe2Fgxn_8HJ6NrtJtp69YqXwocutAoa9Q")
        .json(&payload)
        .send()
        .await?;

    let debil = res.json::<serde_json::Value>().await.unwrap();
    return Ok(debil["idToken"].as_str().unwrap().to_string());
}

pub async fn send_verification_code(
    client: &reqwest::Client,
    id_token: &str,
    number: &str,
    country: &str,
) {
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
        headers.insert(AUTHORIZATION, format!("Bearer {id_token}").parse().unwrap());
        headers
    };
    let _res = client
        .post("https://super-account.spapp.zabka.pl/")
        .json(&payload)
        .headers(headers)
        .send()
        .await;
    // println!("{}", res.json::<serde_json::Value>().unwrap());
}

pub async fn phone_auth(
    client: &reqwest::Client,
    id_token: &str,
    number: &str,
    country: &str,
    verification_code: &str,
) -> Result<String, reqwest::Error> {
    let headers = {
        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert(AUTHORIZATION, format!("Bearer {id_token}").parse().unwrap());
        headers.insert(USER_AGENT, "okhttp/4.12.0".parse().unwrap());
        headers.insert(
            "x-apollo-operation-id",
            "a531998ec966db0951239efb91519560346cfecac77459fe3b85c5b786fa41de"
                .parse()
                .unwrap(),
        );
        headers.insert(
            "x-apollo-operation-name",
            "SignInWithPhone".parse().unwrap(),
        );
        headers.insert(
            ACCEPT,
            "multipart/mixed; deferSpec=20220824, application/json"
                .parse()
                .unwrap(),
        );
        headers.insert(CONTENT_LENGTH, "250".parse().unwrap());
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

    let res = client
        .post("https://super-account.spapp.zabka.pl/")
        .json(&payload)
        .headers(headers)
        .send()
        .await?;

    let debil = res.json::<serde_json::Value>().await.unwrap();
    return Ok(debil["data"]["signIn"]["customToken"]
        .as_str()
        .unwrap()
        .to_string());
}

pub async fn verify_custom_token(
    client: &reqwest::Client,
    custom_token: &str,
) -> Result<String, reqwest::Error> {
    let payload = json!({
        "token": custom_token,
        "returnSecureToken": "True",
    });

    let res = client.post("https://www.googleapis.com/identitytoolkit/v3/relyingparty/verifyCustomToken?key=AIzaSyDe2Fgxn_8HJ6NrtJtp69YqXwocutAoa9Q")
        .json(&payload)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;

    let debil = res.json::<serde_json::Value>().await.unwrap();

    return Ok(debil["idToken"].as_str().unwrap().to_string());
}

pub async fn get_user_secrets(
    client: &reqwest::Client,
    custom_token: &str,
) -> Result<serde_json::Value, reqwest::Error> {
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
        headers.insert("Cache-Control", "no-cache".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(
            USER_AGENT,
            "Zappka/40038 (Horizon; nintendo/ctr; 56c41945-ba88-4543-a525-4e8f7d4a5812) REL/28"
                .parse()
                .unwrap(),
        );
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers.insert(AUTHORIZATION, custom_token.parse().unwrap());
        headers
    };

    let res = client
        .post("https://api.spapp.zabka.pl/")
        .json(&payload)
        .headers(headers)
        .send()
        .await?;
    return Ok(res.json::<serde_json::Value>().await.unwrap()["data"]["qrCode"].clone());
    // return (data["loyalSecret"].as_str().unwrap().to_string(), data["paySecret"].as_str().unwrap().to_string(), data["ployId"].as_str().unwrap().to_string())
}

pub async fn get_nano_status(
    client: &reqwest::Client,
    custom_token: &str,
) -> Result<bool, reqwest::Error> {
    let payload = json!({
        "operationName": "PaymentCards",
        "query": "query PaymentCards { paymentCards { paymentCards { id isDefault lastFourDigits } } }",
        "variables": {}
    });

    let headers = {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Cache-Control", "no-cache".parse().unwrap());
        headers.insert(
            USER_AGENT,
            "Zappka/40038 (Horizon; nintendo/ctr; 56c41945-ba88-4543-a525-4e8f7d4a5812) REL/28"
                .parse()
                .unwrap(),
        );
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers.insert(AUTHORIZATION, custom_token.parse().unwrap());
        headers
    };

    let res = client
        .post("https://api.spapp.zabka.pl/")
        .json(&payload)
        .headers(headers)
        .send()
        .await?;

    let cards =
        &res.json::<serde_json::Value>().await.unwrap()["data"]["paymentCards"]["paymentCards"];

    if cards.is_array() && !cards.as_array().unwrap().is_empty() {
        return Ok(true);
    }
    return Ok(false);
}

// #[tokio::main(flavor = "current_thread")]
// async fn main() {
//     let args = Args::parse();
//     let client = reqwest::Client::new();

//     let id_token = get_token_id(&client).await.unwrap();
//     send_verification_code(&client, &id_token.as_str(), &args.number, &args.country).await;

//     use std::io;
//     let mut verification_code = String::new();
//     io::stdin().read_line(&mut verification_code).unwrap();
//     let custom_token = phone_auth(&client, &id_token.as_str(), &args.number, &args.country, verification_code.trim_end()).await.unwrap();
//     let identity_provider_token = verify_custom_token(&client, custom_token.as_str()).await.unwrap();
//     // println!("{}", identity_provider_token);
//     println!("{}", get_user_secrets(&client, identity_provider_token.as_str()).await.unwrap());
//     println!("{}", get_nano_status(&client, identity_provider_token.as_str()).await.unwrap());
// }
