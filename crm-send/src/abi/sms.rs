use super::{to_ts, Sender};
use crate::{
    pb::{SendRequest, SendResponse, SmsMessage},
    Message, NotificationService,
};
use tonic::Status;
use tracing::warn;

impl Sender for SmsMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status> {
        let message_id = self.message_id.clone();
        svc.sender.send(Message::Sms(self)).await.map_err(|e| {
            warn!("Failed to send message: {:?}", e);
            Status::internal("Failed to send message")
        })?;
        Ok(SendResponse {
            message_id,
            timestamp: Some(to_ts()),
        })
    }
}

impl From<SmsMessage> for Message {
    fn from(sms: SmsMessage) -> Self {
        Self::Sms(sms)
    }
}

impl From<SmsMessage> for SendRequest {
    fn from(sms: SmsMessage) -> Self {
        Self {
            message: Some(sms.into()),
        }
    }
}

#[cfg(feature = "test_utils")]
impl SmsMessage {
    pub fn fake() -> Self {
        use fake::{faker::phone_number::zh_cn::PhoneNumber, Fake};
        use uuid::Uuid;
        Self {
            message_id: Uuid::new_v4().to_string(),
            sender: PhoneNumber().fake(),
            reciepients: vec![PhoneNumber().fake()],
            body: "Hello, world!".to_string(),
        }
    }
}
