use reqwest::Method;
use crate::client::WhatsAppClient;
use crate::error::Result;
use crate::models::inbound::UrlDeletionRequest;

pub struct MediaApi<'a> {
    client: &'a WhatsAppClient,
}

impl<'a> MediaApi<'a> {
    pub(crate) fn new(client: &'a WhatsAppClient) -> Self {
        Self { client }
    }

    pub async fn delete_media(&self, sender: &str, url: &str) -> Result<()> {
        let path = format!("/whatsapp/1/senders/{sender}/media");
        let body = UrlDeletionRequest { url: url.to_string() };
        self.client.request_no_content(Method::DELETE, &path, Some(&body)).await
    }
}
