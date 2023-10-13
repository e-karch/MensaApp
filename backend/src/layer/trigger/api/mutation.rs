//! See [`MutationRoot`].
use crate::util::{ReportReason, Uuid};
use async_graphql::{Context, Object, Result, Upload};
use tokio::fs::File;
use tracing::{instrument, trace};

use super::util::ApiUtil;

/// Class implementing `GraphQLs` root mutations.
#[derive(Debug)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// This mutation adds an image to the specified main dish.
    /// The user has to be authenticated.
    ///
    /// `image_url` is a link to a Flickr image used to get information about it.
    ///
    /// If the meal does not exist, or the URL does not lead to Flickr
    /// or the image is not licenced under a [CC0](https://creativecommons.org/publicdomain/zero/1.0/) licence
    /// or another error occurred while adding the image an error message will be returned.
    ///
    /// If the image was added is successful, `true` is returned.
    #[instrument(skip(self, ctx, image), fields(file_name = image.value(ctx)?.filename, file_type = image.value(ctx)?.content_type))]
    // todo add auth info to tracing
    async fn add_image(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Id of the meal to link an image to.")] meal_id: Uuid,
        #[graphql(desc = "The image itself as multipart attachment.")] image: Upload,
        #[graphql(desc = "Sha512 hash of the uploaded image file. Encoded as Base64.")]
        hash: String,
    ) -> Result<bool> {
        ctx.check_authentication()?;

        trace!("Mutated `addImage`");
        let command = ctx.get_command();
        let client_id = ctx.get_client_id()?;
        let upload = image.value(ctx)?;
        // todo check hash
        // todo read to image directly
        command
            .add_image(
                meal_id,
                upload.content_type,
                File::from_std(upload.content),
                client_id,
            )
            .await?;
        Ok(true)
    }

    /// This mutation either adds a rating to the specified main dish (if no such rating existed), or modifies an existing one.
    /// The user has to be authenticated.
    /// If the main dish does not exist, or any other error occurs in the process, an error message is returned.
    /// If the rating was successfully added or changed, 'true' is returned.
    #[instrument(skip(self, ctx))]
    async fn set_rating(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Id of the meal to rate to.")] meal_id: Uuid,
        #[graphql(desc = "The new rating of the main dish.")] rating: u32,
    ) -> Result<bool> {
        ctx.check_authentication()?;

        trace!("Mutated `setRating`");
        let command = ctx.get_command();
        let client_id = ctx.get_client_id()?;

        command.set_meal_rating(meal_id, rating, client_id).await?;
        Ok(true)
    }

    /// This mutation adds an upvote to the specified image.
    /// The user has to be authenticated.
    /// If the image does not exist, or any other error occurs in the process, an error message is returned.
    /// If the upvote was successfully added, 'true' is returned.
    #[instrument(skip(self, ctx))]
    async fn add_upvote(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Id of the image to add the upvote to.")] image_id: Uuid,
    ) -> Result<bool> {
        ctx.check_authentication()?;

        trace!("Mutated `addUpvote`");
        let command = ctx.get_command();
        let client_id = ctx.get_client_id()?;

        command.add_image_upvote(image_id, client_id).await?;
        Ok(true)
    }

    /// This mutation removes the upvote from the specified image.
    /// The user has to be authenticated.
    /// If the image does not exist, or any other error occurs in the process, an error message is returned.
    /// If the upvote was successfully removed, 'true' is returned.
    #[instrument(skip(self, ctx))]
    async fn remove_upvote(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Id of the image to remove the upvote from.")] image_id: Uuid,
    ) -> Result<bool> {
        ctx.check_authentication()?;

        trace!("Mutated `removeUpvote`");
        let command = ctx.get_command();
        let client_id = ctx.get_client_id()?;

        command.remove_image_upvote(image_id, client_id).await?;
        Ok(true)
    }

    /// This mutation adds a downvote to the specified image.
    /// The user has to be authenticated.
    /// If the image does not exist, or any other error occurs in the process, an error message is returned.
    /// If the downvote was successfully added, 'true' is returned.
    #[instrument(skip(self, ctx))]
    async fn add_downvote(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Id of the image to add the downvote to.")] image_id: Uuid,
    ) -> Result<bool> {
        ctx.check_authentication()?;

        trace!("Mutated `addDownvote`");
        let command = ctx.get_command();
        let client_id = ctx.get_client_id()?;

        command.add_image_downvote(image_id, client_id).await?;
        Ok(true)
    }

    /// This mutation removes the downvote from the specified image.
    /// The user has to be authenticated.
    /// If the image does not exist, or any other error occurs in the process, an error message is returned.
    /// If the downvote was successfully removed, 'true' is returned.
    #[instrument(skip(self, ctx))]
    async fn remove_downvote(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Id of the image to remove the downvote from.")] image_id: Uuid,
    ) -> Result<bool> {
        ctx.check_authentication()?;

        trace!("Mutated `removeDownvote`");
        let command = ctx.get_command();
        let client_id = ctx.get_client_id()?;

        command.remove_image_downvote(image_id, client_id).await?;
        Ok(true)
    }

    /// This mutation adds a report to the specified image.
    /// The user has to be authenticated.
    /// If the image does not exist, or any other error occurs in the process, an error message is returned.
    /// If the report was successfully added, 'true' is returned.
    #[instrument(skip(self, ctx))]
    async fn report_image(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "Id of the image to report.")] image_id: Uuid,
        #[graphql(desc = "The reason for reporting the image.")] reason: ReportReason,
    ) -> Result<bool> {
        ctx.check_authentication()?;

        trace!("Mutated `reportImage`");
        let command = ctx.get_command();
        let client_id = ctx.get_client_id()?;

        command.report_image(image_id, reason, client_id).await?;
        Ok(true)
    }
}
