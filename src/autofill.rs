use regex::Regex;
use zbus::message::Type;
use zbus::{Connection, MessageStream, MatchRule};
use zbus::fdo::DBusProxy;
use futures_util::stream::StreamExt;
use zvariant::ObjectPath;

#[tokio::main(flavor = "current_thread")]
async fn main() -> zbus::Result<()> {
    let connection = Connection::system().await?;

    let match_rule = MatchRule::builder()
        .msg_type(Type::Signal)
        .interface("org.freedesktop.ModemManager1.Modem.Messaging")?
        .member("Added")?
        .build();

    let dbus_proxy = DBusProxy::new(&connection).await?;
    dbus_proxy.add_match_rule(match_rule).await?;

    println!("Listening for Added signals...");

    let mut stream = MessageStream::from(&connection);

    while let Some(msg) = stream.next().await {
        let msg = msg?;

        let interface = msg.header().interface().cloned();
        let member = msg.header().member().cloned();

        if matches!(interface, Some(i) if i.as_str() == "org.freedesktop.ModemManager1.Modem.Messaging") &&
           matches!(member, Some(m) if m.as_str() == "Added")
        {
            let body_data = msg.body();
            let (path, flag): (ObjectPath, bool) = body_data.deserialize()?;
            println!("Received path: {path}, flag: {flag}");

            match fetch_text_from_path(&connection, &path).await {
                Ok(text) => {
                    println!("Message Text: {text}");
                    let code = get_sms_code(&text).await;

                    if let Some(code) = code {
                        println!("sms code: {code}");
                    }
                    // if !code.is_none(){
                    //     println!("sms code: {}", code.unwrap());
                    // }
                },
                Err(e) => eprintln!("Failed to get text from {path}: {e}"),
            }
        }
    }

    Ok(())
}

async fn fetch_text_from_path(conn: &Connection, path: &ObjectPath<'_>) -> zbus::Result<String>  {
    let proxy = zbus::Proxy::new(
        conn,
        "org.freedesktop.ModemManager1",
        path.clone(),
        "org.freedesktop.ModemManager1.Sms",
    )
    .await?;

    let text: String = proxy.get_property("Text").await?;
    // let code: String = get_sms_code(&text).await.unwrap();
    Ok(text)
}

async fn get_sms_code(sms_text: &str) -> Option<String>{
    if sms_text.contains("dQsUibGhU1V"){
        let re = Regex::new(r"\b\d{6}\b").unwrap();

        match re.find(&sms_text) {
            Some(code) => return Some(code.as_str().to_string()),
            None => None,
        } 
    } else {
        None
    }
}