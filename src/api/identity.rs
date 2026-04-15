use reqwest::Method;
use crate::client::WhatsAppClient;
use crate::error::Result;
use crate::models::identity::*;

pub struct IdentityApi<'a> {
    client: &'a WhatsAppClient,
}

impl<'a> IdentityApi<'a> {
    pub(crate) fn new(client: &'a WhatsAppClient) -> Self {
        Self { client }
    }

    pub async fn get_identity(
        &self,
        sender: &str,
        user_number: &str,
    ) -> Result<IdentityInfo> {
        let path = format!("/whatsapp/1/{sender}/contacts/{user_number}/identity");
        self.client.request(Method::GET, &path, None::<&()>).await
    }

    pub async fn confirm_identity(
        &self,
        sender: &str,
        user_number: &str,
        confirmation: &IdentityConfirmation,
    ) -> Result<()> {
        let path = format!("/whatsapp/1/{sender}/contacts/{user_number}/identity");
        self.client
            .request_no_content(Method::PUT, &path, Some(confirmation))
            .await
    }

    pub async fn delete_identity(&self, sender: &str, user_number: &str) -> Result<()> {
        let path = format!("/whatsapp/1/{sender}/contacts/{user_number}/identity");
        self.client.request_no_content(Method::DELETE, &path, None::<&()>).await
    }
}
