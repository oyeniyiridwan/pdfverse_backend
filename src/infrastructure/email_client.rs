use crate::{app::settings::Settings, utils::error::ApiError};
use lettre::{
    Address, Message, SmtpTransport, Transport,
    message::{Mailbox, header},
    transport::smtp::authentication::Credentials,
};

#[derive(Clone)]
pub struct EmailClient {
    mailer: SmtpTransport,
    from: Mailbox,
}

impl EmailClient {
    pub fn new() -> Result<Self, ApiError> {
        let settings = Settings::from_env();
        let from_email_address = settings
            .mail_address
            .parse::<Address>()
            .map_err(|e| ApiError::internal_msg(format!("EmailClient : {e}")))?;
        let from = Mailbox {
            name: Some("Shevyverse Support".to_string()),
            email: from_email_address,
        };
        let credential = Credentials::new(settings.mail_address, settings.mail_password);

        let mailer = SmtpTransport::relay(&settings.smtp_url)
            .map_err(|e| ApiError::internal_msg(format!("EmailClient-mailer: {e}")))?
            .credentials(credential)
            .build();

        Ok(Self {
            mailer: mailer,
            from: from,
        })
    }



pub fn send_email(
    &self,
    recipient_email: &str,
    subject: &str,
    html_body: String
) -> Result<(), ApiError> {
    let to_email_address = recipient_email
        .parse::<Address>()
        .map_err(|e| ApiError::internal_msg(format!("send_email: {e}")))?;
    let to = Mailbox {
        name: None,
        email: to_email_address,
    };

    let email = Message::builder()
        .from(self.from.clone())
        .to(to)
        .subject(subject)
        .header(header::ContentType::TEXT_HTML)
        .body(html_body)
        .map_err(|e| ApiError::internal_msg(format!("send_email-email: {e}")))?;

     self.mailer.send(&email)            
.map_err(|e| ApiError::internal_msg(format!("send_email-self.mailer: {e}")))?;


    Ok(())
}

}

