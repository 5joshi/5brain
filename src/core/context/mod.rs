use std::sync::Arc;

use twilight_gateway::{cluster::Events, Cluster};
use twilight_http::{client::InteractionClient, Client};
use twilight_model::{
    channel::message::allowed_mentions::AllowedMentionsBuilder,
    id::{marker::ApplicationMarker, Id},
};
use twilight_standby::Standby;

use crate::{core::CONFIG, custom_client::CustomClient, BotResult};

use super::{cluster::build_cluster, Cache};

pub struct Context {
    pub cache: Cache,
    pub cluster: Cluster,
    pub http: Arc<Client>,
    pub standby: Standby,
    pub client: CustomClient,
    data: ContextData,
}

impl Context {
    pub fn interaction(&self) -> InteractionClient<'_> {
        self.http.interaction(self.data.application_id)
    }

    pub async fn new() -> BotResult<(Self, Events)> {
        let config = CONFIG.get().unwrap();
        let discord_token = &config.discord_token;

        let mentions = AllowedMentionsBuilder::new()
            .replied_user()
            .roles()
            .users()
            .build();

        // Connect to the discord http client
        let http = Client::builder()
            .token(discord_token.to_owned())
            .remember_invalid_token(false)
            .default_allowed_mentions(mentions)
            .build();

        let http = Arc::new(http);

        let current_user = http.current_user().exec().await?.model().await?;
        let application_id = current_user.id.cast();

        info!(
            "Connecting to Discord as {}#{}...",
            current_user.name, current_user.discriminator
        );

        let client = CustomClient::new(config).await?;

        let data = ContextData::new(application_id).await?;
        let (cache, resume_data) = Cache::new().await;

        let (cluster, events) =
            build_cluster(discord_token, Arc::clone(&http), resume_data).await?;

        let ctx = Self {
            cache,
            http,
            client,
            cluster,
            data,
            standby: Standby::new(),
        };

        Ok((ctx, events))
    }
}

struct ContextData {
    application_id: Id<ApplicationMarker>,
}

impl ContextData {
    async fn new(application_id: Id<ApplicationMarker>) -> BotResult<Self> {
        Ok(Self { application_id })
    }
}
