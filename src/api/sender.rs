use reqwest::Method;
use crate::client::WhatsAppClient;
use crate::error::Result;
use crate::models::sender::*;

pub struct SenderApi<'a> {
    client: &'a WhatsAppClient,
}

impl<'a> SenderApi<'a> {
    pub(crate) fn new(client: &'a WhatsAppClient) -> Self {
        Self { client }
    }

    pub async fn get_quality(&self, senders: &[&str]) -> Result<WhatsAppSenderQualityResponse> {
        let query = senders.join(",");
        let path = format!("/whatsapp/1/senders/quality?sender={query}");
        self.client.request(Method::GET, &path, None::<&()>).await
    }

    pub async fn get_business_info(&self, sender: &str) -> Result<BusinessInfoResponse> {
        let path = format!("/whatsapp/1/senders/{sender}/business-info");
        self.client.request(Method::GET, &path, None::<&()>).await
    }

    pub async fn update_business_info(
        &self,
        sender: &str,
        request: &BusinessInfoRequest,
    ) -> Result<()> {
        let path = format!("/whatsapp/1/senders/{sender}/business-info");
        self.client.request_no_content(Method::PATCH, &path, Some(request)).await
    }

    pub async fn get_logo(&self, sender: &str) -> Result<Vec<u8>> {
        let path = format!("/whatsapp/1/senders/{sender}/business-info/logo");
        self.client.request_bytes(Method::GET, &path).await
    }

    pub async fn get_calling_permissions(
        &self,
        sender: &str,
        user_number: &str,
    ) -> Result<CallingPermissionResponse> {
        let path = format!("/whatsapp/1/senders/{sender}/calls/{user_number}/permissions");
        self.client.request(Method::GET, &path, None::<&()>).await
    }

    pub async fn get_public_key(&self, sender: &str) -> Result<SenderPublicKeyResponse> {
        let path = format!("/whatsapp/1/senders/{sender}/public-key");
        self.client.request(Method::GET, &path, None::<&()>).await
    }

    pub async fn upload_public_key(&self, sender: &str, pem_data: Vec<u8>) -> Result<()> {
        let path = format!("/whatsapp/1/senders/{sender}/public-key");
        let url = self.client.base_url().join(&path)?;
        let part = reqwest::multipart::Part::bytes(pem_data).file_name("public_key.pem");
        let form = reqwest::multipart::Form::new().part("file", part);
        let mut req = self.client.http_client().request(Method::POST, url);
        req = self.client.auth().apply(req);
        req = req.multipart(form);
        let resp = req.send().await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            let code = resp.status().as_u16();
            let text = resp.text().await.unwrap_or_default();
            Err(crate::InfobipError::Api(
                crate::error::ApiError::from_status_and_body(code, &text),
            ))
        }
    }
}
