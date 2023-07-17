use async_trait::async_trait;
use chrono::Local;
use std::marker::{Send, Sync};

use crate::{
    interface::{
        admin_notification::{AdminNotification, ImageReportInfo},
        api_command::{AuthInfo, Command, CommandError, Result},
        image_hoster::ImageHoster,
        persistent_data::{model::ImageInfo, CommandDataAccess},
    },
    layer::logic::api_command::auth::{authenticator::Authenticator, command_type::CommandType},
    util::{Date, ReportReason, Uuid},
};

const REPORT_FACTOR: f64 = 1.0 / 35.0;

pub struct CommandHandler<D, A, I>
where
    D: CommandDataAccess + Sync + Send,
    A: AdminNotification + Sync + Send,
    I: ImageHoster + Sync + Send,
{
    command_data: D,
    admin_notification: A,
    image_hoster: I,
    auth: Authenticator,
}

impl<D, A, I> CommandHandler<D, A, I>
where
    D: CommandDataAccess + Sync + Send,
    A: AdminNotification + Sync + Send,
    I: ImageHoster + Sync + Send,
{
    /// A function that creates a new [`CommandHandler`]
    ///
    /// # Errors
    /// Returns an error, if the api keys could not be gotten from [`command_data`]
    pub async fn new(command_data: D, admin_notification: A, image_hoster: I) -> Result<Self> {
        let keys: Vec<String> = command_data
            .get_api_keys()
            .await?
            .into_iter()
            .map(|x| x.key)
            .collect();
        Ok(Self {
            command_data,
            admin_notification,
            image_hoster,
            auth: Authenticator::new(keys),
        })
    }

    fn will_be_hidden(image: &ImageInfo) -> bool {
        Self::days_since(image.upload_date) <= 30
            && image.report_count > Self::get_report_barrier(image.upload_date)
    }

    fn days_since(date: Date) -> i64 {
        let today = Local::now().date_naive();
        let difference = today.signed_duration_since(date);
        difference.num_days()
    }

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    fn get_report_barrier(date: Date) -> u32 {
        let t = Self::days_since(date) as f64;
        REPORT_FACTOR.mul_add(t * t, 5.0).floor() as u32
    }
}

#[async_trait]
impl<D, A, I> Command for CommandHandler<D, A, I>
where
    D: CommandDataAccess,
    A: AdminNotification,
    I: ImageHoster,
{
    async fn report_image(
        &self,
        image_id: Uuid,
        reason: ReportReason,
        auth_info: AuthInfo,
    ) -> Result<()> {
        let auth_info = auth_info.ok_or(CommandError::NoAuth)?;

        let command_type = CommandType::ReportImage { image_id, reason };
        self.auth.authn_command(&auth_info, &command_type)?;
        let mut info = self.command_data.get_image_info(image_id).await?;
        if !info.approved {
            info.report_count += 1;
            self.command_data
                .add_report(image_id, auth_info.client_id, reason)
                .await?;
            let will_be_hidden = Self::will_be_hidden(&info);
            if will_be_hidden {
                self.command_data.hide_image(image_id).await?;
            }
            let report_info = ImageReportInfo {
                reason,
                image_id,
                image_got_hidden: will_be_hidden,
                image_link: info.image_url,
                report_count: info.report_count,
                positive_rating_count: info.positive_rating_count,
                negative_rating_count: info.negative_rating_count,
                get_image_rank: info.image_rank,
            };

            self.admin_notification
                .notify_admin_image_report(report_info)
                .await;
        }
        Ok(())
    }

    async fn add_image_upvote(&self, image_id: Uuid, auth_info: AuthInfo) -> Result<()> {
        let auth_info = auth_info.ok_or(CommandError::NoAuth)?;
        let command_type = CommandType::AddUpvote { image_id };
        self.auth.authn_command(&auth_info, &command_type)?;
        self.command_data
            .add_upvote(image_id, auth_info.client_id)
            .await?;
        Ok(())
    }

    async fn add_image_downvote(&self, image_id: Uuid, auth_info: AuthInfo) -> Result<()> {
        let auth_info = auth_info.ok_or(CommandError::NoAuth)?;
        let command_type = CommandType::AddDownvote { image_id };
        self.auth.authn_command(&auth_info, &command_type)?;
        self.command_data
            .add_downvote(image_id, auth_info.client_id)
            .await?;
        Ok(())
    }

    async fn remove_image_upvote(&self, image_id: Uuid, auth_info: AuthInfo) -> Result<()> {
        let auth_info = auth_info.ok_or(CommandError::NoAuth)?;
        let command_type = CommandType::RemoveUpvote { image_id };
        self.auth.authn_command(&auth_info, &command_type)?;
        self.command_data
            .remove_upvote(image_id, auth_info.client_id)
            .await?;
        Ok(())
    }

    async fn remove_image_downvote(&self, image_id: Uuid, auth_info: AuthInfo) -> Result<()> {
        let auth_info = auth_info.ok_or(CommandError::NoAuth)?;
        let command_type = CommandType::RemoveDownvote { image_id };
        self.auth.authn_command(&auth_info, &command_type)?;
        self.command_data
            .remove_downvote(image_id, auth_info.client_id)
            .await?;
        Ok(())
    }

    async fn add_image(&self, meal_id: Uuid, image_url: String, auth_info: AuthInfo) -> Result<()> {
        let auth_info = auth_info.ok_or(CommandError::NoAuth)?;
        let command_type = CommandType::AddImage {
            meal_id,
            url: image_url.clone(),
        };
        self.auth.authn_command(&auth_info, &command_type)?;
        let image_meta_data = self.image_hoster.validate_url(&image_url).await?;
        let licence_ok = self.image_hoster.check_licence(&image_meta_data.id).await?;
        if licence_ok {
            self.command_data
                .link_image(
                    meal_id,
                    auth_info.client_id,
                    image_meta_data.id,
                    image_meta_data.image_url,
                )
                .await?;
        }
        Ok(())
    }

    async fn set_meal_rating(&self, meal_id: Uuid, rating: u32, auth_info: AuthInfo) -> Result<()> {
        let auth_info = auth_info.ok_or(CommandError::NoAuth)?;
        let command_type = CommandType::SetRating { meal_id, rating };
        self.auth.authn_command(&auth_info, &command_type)?;
        self.command_data
            .add_rating(meal_id, auth_info.client_id, rating)
            .await?;
        Ok(())
    }
}
