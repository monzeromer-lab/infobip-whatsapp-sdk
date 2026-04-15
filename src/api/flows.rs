use reqwest::Method;
use crate::client::WhatsAppClient;
use crate::error::Result;
use crate::models::flows::*;

pub struct FlowsApi<'a> {
    client: &'a WhatsAppClient,
}

impl<'a> FlowsApi<'a> {
    pub(crate) fn new(client: &'a WhatsAppClient) -> Self {
        Self { client }
    }

    pub async fn get_flows(&self, sender: &str) -> Result<GetSenderFlowsResponse> {
        let path = format!("/whatsapp/1/senders/{sender}/flows");
        self.client.request(Method::GET, &path, None::<&()>).await
    }

    pub async fn create_flow(
        &self,
        sender: &str,
        request: &CreateFlowRequest,
    ) -> Result<FlowDataResponse> {
        let path = format!("/whatsapp/1/senders/{sender}/flows");
        self.client.request(Method::POST, &path, Some(request)).await
    }

    pub async fn update_flow(
        &self,
        sender: &str,
        flow_id: &str,
        request: &UpdateFlowRequest,
    ) -> Result<()> {
        let path = format!("/whatsapp/1/senders/{sender}/flows/{flow_id}");
        self.client.request_no_content(Method::PATCH, &path, Some(request)).await
    }

    pub async fn delete_flow(&self, sender: &str, flow_id: &str) -> Result<()> {
        let path = format!("/whatsapp/1/senders/{sender}/flows/{flow_id}");
        self.client.request_no_content(Method::DELETE, &path, None::<&()>).await
    }

    pub async fn publish_flow(&self, sender: &str, flow_id: &str) -> Result<()> {
        let path = format!("/whatsapp/1/senders/{sender}/flows/{flow_id}/publish");
        self.client.request_no_content(Method::POST, &path, None::<&()>).await
    }

    pub async fn deprecate_flow(&self, sender: &str, flow_id: &str) -> Result<()> {
        let path = format!("/whatsapp/1/senders/{sender}/flows/{flow_id}/deprecate");
        self.client.request_no_content(Method::POST, &path, None::<&()>).await
    }

    pub async fn get_flow_json(&self, sender: &str, flow_id: &str) -> Result<GetFlowJsonResponse> {
        let path = format!("/whatsapp/1/senders/{sender}/flows/{flow_id}/json");
        self.client.request(Method::GET, &path, None::<&()>).await
    }

    pub async fn update_flow_json(
        &self,
        sender: &str,
        flow_id: &str,
        json: &serde_json::Value,
    ) -> Result<AddFlowJsonResponse> {
        let path = format!("/whatsapp/1/senders/{sender}/flows/{flow_id}/json");
        self.client.request(Method::PUT, &path, Some(json)).await
    }

    pub async fn get_flow_preview(&self, sender: &str, flow_id: &str) -> Result<FlowPreview> {
        let path = format!("/whatsapp/1/senders/{sender}/flows/{flow_id}/preview");
        self.client.request(Method::GET, &path, None::<&()>).await
    }

    pub async fn generate_flow(
        &self,
        sender: &str,
        request: &GenerateFlowRequest,
    ) -> Result<GenerateFlowResponse> {
        let path = format!("/whatsapp/1/senders/{sender}/flows/generate");
        self.client.request(Method::POST, &path, Some(request)).await
    }
}
