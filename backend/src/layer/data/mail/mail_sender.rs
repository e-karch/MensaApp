use async_trait::async_trait;
use thiserror::Error;

use lettre::{
    address::AddressError, message::Mailbox, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};

use crate::{
    interface::admin_notification::{AdminNotification, ImageReportInfo},
    layer::data::mail::mail_info::MailInfo,
};

use string_template::Template;
use tracing::{info, warn};

pub type MailResult<T> = std::result::Result<T, MailError>;

const REPORT_TEMPLATE: &str = include_str!("./template.txt");

/// Enum describing the possible ways, the mail notification can fail.
#[derive(Debug, Error)]
pub enum MailError {
    #[error("an error occurred while parsing the addresses: {0}")]
    AddressError(#[from] AddressError),
    #[error("an error occurred while reading the template: {0}")]
    TemplateError(#[from] std::io::Error),
    #[error("an error occurred while parsing the mail: {0}")]
    MailParseError(#[from] lettre::error::Error),
    #[error("an error occurred while sending the mail: {0}")]
    MailSendError(#[from] lettre::transport::smtp::Error),
}

pub struct MailSender {
    config: MailInfo,
    mailer: SmtpTransport,
}

#[async_trait]
impl AdminNotification for MailSender {
    async fn notify_admin_image_report(&self, info: ImageReportInfo) {
        if let Err(error) = self.try_notify_admin_image_report(&info) {
            warn!("{error:?}");
        }
    }
}

impl MailSender {
    /// Creates a new [`MailSender`] with the attributes defined in config. Also creates an SMTP connection to the smtp server defined in config
    ///
    /// # Errors
    /// Returns an error, if the connection could not be established to the smtp server
    pub fn new(config: MailInfo) -> MailResult<Self> {
        let creds = Credentials::new(config.username.clone(), config.password.clone());
        let transport_builder = SmtpTransport::relay(&config.smtp_server)?;
        let mailer = transport_builder
            .port(config.smtp_port)
            .credentials(creds)
            .build();
        Ok(Self { config, mailer })
    }

    fn try_notify_admin_image_report(&self, info: &ImageReportInfo) -> MailResult<()> {
        let sender = self.get_sender()?;
        let reciever = self.get_reciever()?;
        let report = Self::get_report(info);
        let email = Message::builder()
            .from(sender)
            .to(reciever)
            .subject("An image was reported and requires your review")
            .body(report)?;
        self.mailer.send(&email)?;
        info!(
            "Email sent successfully for image with id {} at {}",
            info.image_id, info.image_link
        );
        Ok(())
    }

    fn get_sender(&self) -> MailResult<Mailbox> {
        format!("MensaKa <{}>", self.config.username.clone())
            .parse()
            .map_err(MailError::AddressError)
    }

    fn get_reciever(&self) -> MailResult<Mailbox> {
        format!("Administrator <{}>", self.config.admin_email_address)
            .parse()
            .map_err(MailError::AddressError)
    }

    fn get_report(info: &ImageReportInfo) -> String {
        let a: [(&str, &dyn ToString); 8] = [
            ("image_link", &info.image_link),
            ("image_id", &info.image_id),
            ("report_count", &info.report_count),
            ("reason", &info.reason),
            ("image_got_hidden", &info.image_got_hidden),
            ("positive_rating_count", &info.positive_rating_count),
            ("negative_rating_count", &info.negative_rating_count),
            ("get_image_rank", &info.get_image_rank),
        ];

        let map = a
            .into_iter()
            .map(|(a, b)| (a, b.to_string()))
            .collect::<Vec<_>>();
        let map = map.iter().map(|(a, b)| (*a, b.as_str())).collect();

        Template::new(REPORT_TEMPLATE).render(&map)
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]
    use crate::{
        interface::admin_notification::ImageReportInfo, layer::data::mail::mail_info::MailInfo,
        layer::data::mail::mail_sender::MailSender, util::Uuid,
    };
    use dotenvy;
    use std::env::{self, VarError};

    const SMTP_SERVER_ENV_NAME: &str = "SMTP_SERVER";
    const SMTP_PORT_ENV_NAME: &str = "SMTP_PORT";
    const SMTP_USERNAME_ENV_NAME: &str = "SMTP_USERNAME";
    const SMTP_PASSWORD_ENV_NAME: &str = "SMTP_PASSWORD";
    const ADMIN_EMAIL_ENV_NAME: &str = "ADMIN_EMAIL";

    #[tokio::test]
    async fn test_notify_admin_image_report() {
        let mail_info = get_mail_info().unwrap();
        let mail_sender = MailSender::new(mail_info).unwrap();
        assert!(mail_sender.mailer.test_connection().unwrap());

        let report_info = ImageReportInfo {
            reason: crate::util::ReportReason::Advert,
            image_got_hidden: true,
            image_id: Uuid::default(),
            image_link: String::from("www.test.com"),
            report_count: 1,
            positive_rating_count: 10,
            negative_rating_count: 20,
            get_image_rank: 1.0,
        };
        if let Err(error) = mail_sender.try_notify_admin_image_report(&report_info) {
            println!("{error}");
            panic!();
        }
    }

    fn get_mail_info() -> Result<MailInfo, VarError> {
        dotenvy::dotenv().ok();
        Ok(MailInfo {
            smtp_server: env::var(SMTP_SERVER_ENV_NAME)?,
            smtp_port: env::var(SMTP_PORT_ENV_NAME)?.parse().unwrap(),
            username: env::var(SMTP_USERNAME_ENV_NAME)?,
            password: env::var(SMTP_PASSWORD_ENV_NAME)?,
            admin_email_address: env::var(ADMIN_EMAIL_ENV_NAME)?,
        })
    }
}
