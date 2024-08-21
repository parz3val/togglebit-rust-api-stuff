use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn create_smtp_client(smtp_username: String, smtp_password: String) -> SmtpTransport {
    let creds = Credentials::new(smtp_username, smtp_password);
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();
    return mailer;
}

type MailParseError = &'static str;

pub fn new_mailbox(address: String) -> Result<Mailbox, MailParseError> {
    match address.parse::<Mailbox>() {
        Ok(result) => Ok(result),
        Err(_) => Err("Failed to parse the mail address"),
    }
}

pub fn build_message(
    sender: Mailbox,
    rx: Mailbox,
    sub: &str,
    content: String,
) -> Result<Message, MailParseError> {
    return match Message::builder()
        .from(sender.clone())
        .reply_to(sender)
        .to(rx)
        .subject(sub)
        .header(ContentType::TEXT_HTML)
        .body(content)
    {
        Ok(m) => Ok(m),
        Err(_) => Err("Failed to build message"),
    };
}

type MailSendError = &'static str;

async fn send_mail(m: &Message, s: &SmtpTransport) -> Result<String, MailSendError> {
    return match s.send(m) {
        Ok(_) => Ok("Sent succesfully".to_owned()),
        Err(e) => {
            dbg!(e);
            Err("Failed to send the message")
        },
    };
}
pub async fn send_smtp_email(sender: String, rx: String, body: String, sub: &str, s: &SmtpTransport) -> Result<String, MailSendError> {
    let sender = new_mailbox(sender)?;
    let rx = new_mailbox(rx)?;
    let m = build_message(sender, rx, sub, body)?;
    return match send_mail(&m, s).await {
        Ok(s) => {Ok(s)},
        Err(e) => {Err(e)}
    };

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_build_message() {
        let sender = new_mailbox(String::from("iamtheparzival@gmail.com")).unwrap();
        let rx = new_mailbox(String::from("sudama.harris@gmail.com")).unwrap();
        let body = String::from(
            r#"
        <html>
            <body>
                <h1>Your verification code is 1234 </h1>
            </body>
        </html>
        "#,
        );

        match build_message(sender, rx, "Verification Message", body) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_parse_mailbox() {
        match new_mailbox(String::from("iamtheparzival@gmail.com")) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_send_email() {
        let sender = new_mailbox(String::from("iamtheparzival@gmail.com")).unwrap();
        let rx = new_mailbox(String::from("sudama.harris@gmail.com")).unwrap();
        let body = String::from(
            r#"
        <html>
            <body>
                <h1>Your verification code is 1234 </h1>
            </body>
        </html>
        "#,
        );

        let smtp_client = create_smtp_client(
            String::from("iamtheparzival@gmail.com"),
            String::from("evya xktm bcho sqkl"),
        );
        match build_message(sender, rx, "Verification Message", body) {
            Ok(m) => match send_mail(&m, &smtp_client).await {
                Ok(_) => assert!(true),
                Err(e) => {
                    dbg!(e);
                    assert!(false);
                }
            },
            Err(_) => assert!(false),
        }
    }
}
