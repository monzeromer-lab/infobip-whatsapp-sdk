use reqwest::Method;
use crate::client::WhatsAppClient;
use crate::error::Result;
use crate::models::registration::*;

pub struct RegistrationApi<'a> {
    client: &'a WhatsAppClient,
}

impl<'a> RegistrationApi<'a> {
    pub(crate) fn new(client: &'a WhatsAppClient) -> Self {
        Self { client }
    }

    pub async fn register_sender(
        &self,
        business_account_id: i64,
        request: &PhoneNumberRequest,
    ) -> Result<RegistrationResponse> {
        let path = format!(
            "/whatsapp/1/embedded-signup/registrations/business-account/{business_account_id}/senders"
        );
        self.client.request(Method::POST, &path, Some(request)).await
    }

    pub async fn send_verification(
        &self,
        sender: &str,
        request: &OtpRequest,
    ) -> Result<()> {
        let path = format!(
            "/whatsapp/1/embedded-signup/registrations/senders/{sender}/verification"
        );
        self.client.request_no_content(Method::PUT, &path, Some(request)).await
    }

    pub async fn verify_code(
        &self,
        sender: &str,
        request: &VerifyCodeRequest,
    ) -> Result<()> {
        let path = format!(
            "/whatsapp/1/embedded-signup/registrations/senders/{sender}/verification"
        );
        self.client.request_no_content(Method::POST, &path, Some(request)).await
    }
}
