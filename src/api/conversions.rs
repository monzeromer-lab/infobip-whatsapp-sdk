use reqwest::Method;
use crate::client::WhatsAppClient;
use crate::error::Result;
use crate::models::conversions::*;

pub struct ConversionsApi<'a> {
    client: &'a WhatsAppClient,
}

impl<'a> ConversionsApi<'a> {
    pub(crate) fn new(client: &'a WhatsAppClient) -> Self {
        Self { client }
    }

    pub async fn submit_conversion(&self, request: &ConversionEventRequest) -> Result<()> {
        self.client
            .request_no_content(Method::POST, "/whatsapp/1/conversions", Some(request))
            .await
    }

    pub async fn submit_waba_conversion(&self, request: &WabaConversionEventRequest) -> Result<()> {
        self.client
            .request_no_content(Method::POST, "/whatsapp/1/conversions/waba", Some(request))
            .await
    }
}
