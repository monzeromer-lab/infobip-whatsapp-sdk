use reqwest::Method;
use crate::client::WhatsAppClient;
use crate::error::Result;
use crate::models::payments::PaymentResponse;

pub struct PaymentsApi<'a> {
    client: &'a WhatsAppClient,
}

impl<'a> PaymentsApi<'a> {
    pub(crate) fn new(client: &'a WhatsAppClient) -> Self {
        Self { client }
    }

    pub async fn get_upi_payu_status(
        &self,
        sender: &str,
        payment_id: &str,
    ) -> Result<PaymentResponse> {
        let path = format!("/whatsapp/1/senders/{sender}/payments/upi/payu/{payment_id}");
        self.client.request(Method::GET, &path, None::<&()>).await
    }

    pub async fn get_brazil_status(
        &self,
        sender: &str,
        payment_id: &str,
    ) -> Result<PaymentResponse> {
        let path = format!("/whatsapp/1/senders/{sender}/payments/br/{payment_id}");
        self.client.request(Method::GET, &path, None::<&()>).await
    }

    pub async fn get_upi_status(
        &self,
        sender: &str,
        payment_id: &str,
    ) -> Result<PaymentResponse> {
        let path = format!("/whatsapp/1/senders/{sender}/payments/upi/{payment_id}");
        self.client.request(Method::GET, &path, None::<&()>).await
    }
}
