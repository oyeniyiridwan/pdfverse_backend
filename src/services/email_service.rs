use crate::{infrastructure::email_client::EmailClient};

pub struct EmailService{
    email_client:EmailClient
}

impl EmailService {
    pub fn new(email_client: EmailClient) -> Self {
        Self { email_client }
    }
    
   pub fn send_verification_or_access_link(
    &self,
    recipient_email: &str,
    new_user: bool,
    name: Option<String>,
    url_in_mail: String,
) {
    let (subject, message) = if new_user {
        (
            "Welcome to ShevyVerse",
            "Click the link below to verify your account",
        )
    } else {
        (
            "Welcome back to ShevyVerse!",
            "Click the link below to access your account",
        )
    };

    let name = name.unwrap_or_default();
    let html_body = format!(
        r#"
        <html>
            <body style="font-family: Arial, sans-serif;">
                <h2>{subject}, {name}</h2>
                <p>{message}</p>
                <p>
                    <a href="{url_in_mail}"
                       style="background-color:#007bff;color:white;padding:10px 15px;text-decoration:none;border-radius:5px;">
                       Account
                    </a>
                </p>
                <p>If you didn’t request this, you can safely ignore this email.</p>
            </body>
        </html>
    "#
    );

    // Clone values so they're owned by the async task
    let email_client = self.email_client.clone();
    let recipient_email = recipient_email.to_string();
    let subject = subject.to_string();
    let html_body = html_body.to_string();

    tokio::spawn(async move {
        if let Err(e) = email_client
            .send_email(&recipient_email, &subject, html_body)
            
        {
            eprintln!("Failed to send email: {:?}", e);
        }
    });
}


}

