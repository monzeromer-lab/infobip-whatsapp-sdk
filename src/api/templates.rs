use reqwest::Method;
use crate::client::WhatsAppClient;
use crate::error::Result;
use crate::models::templates::v2::*;

pub struct TemplatesApi<'a> {
    client: &'a WhatsAppClient,
}

impl<'a> TemplatesApi<'a> {
    pub(crate) fn new(client: &'a WhatsAppClient) -> Self {
        Self { client }
    }

    pub async fn get_templates(&self, sender: &str) -> Result<TemplatesApiResponse> {
        let path = format!("/whatsapp/2/senders/{sender}/templates");
        self.client.request(Method::GET, &path, None::<&()>).await
    }

    pub async fn create_template(
        &self,
        sender: &str,
        request: &TemplatePublicApiRequest,
    ) -> Result<TemplateApiResponse> {
        let path = format!("/whatsapp/2/senders/{sender}/templates");
        self.client.request(Method::POST, &path, Some(request)).await
    }

    pub async fn get_template(&self, sender: &str, id: &str) -> Result<TemplateApiResponse> {
        let path = format!("/whatsapp/2/senders/{sender}/templates/{id}");
        self.client.request(Method::GET, &path, None::<&()>).await
    }

    pub async fn edit_template(
        &self,
        sender: &str,
        id: &str,
        request: &TemplateEditRequest,
    ) -> Result<TemplateApiResponse> {
        let path = format!("/whatsapp/2/senders/{sender}/templates/{id}");
        self.client.request(Method::PATCH, &path, Some(request)).await
    }

    pub async fn delete_template(&self, sender: &str, template_name: &str) -> Result<()> {
        let path = format!("/whatsapp/2/senders/{sender}/templates/{template_name}");
        self.client.request_no_content(Method::DELETE, &path, None::<&()>).await
    }

    pub async fn get_all_templates(&self) -> Result<TemplatesApiResponsePaginated> {
        self.client
            .request(Method::GET, "/whatsapp/1/templates", None::<&()>)
            .await
    }

    #[deprecated(note = "Use get_templates (v2) instead")]
    pub async fn get_templates_v1(&self, sender: &str) -> Result<TemplatesApiResponse> {
        let path = format!("/whatsapp/1/senders/{sender}/templates");
        self.client.request(Method::GET, &path, None::<&()>).await
    }

    #[deprecated(note = "Use create_template (v2) instead")]
    pub async fn create_template_v1(
        &self,
        sender: &str,
        request: &TemplatePublicApiRequest,
    ) -> Result<TemplateApiResponse> {
        let path = format!("/whatsapp/1/senders/{sender}/templates");
        self.client.request(Method::POST, &path, Some(request)).await
    }
}
