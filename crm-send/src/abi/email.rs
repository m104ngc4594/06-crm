use super::{to_ts, Sender};
use crate::{
    pb::{EmailMessage, SendRequest, SendResponse},
    Message, NotificationService,
};
use tonic::Status;
use tracing::warn;

impl Sender for EmailMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status> {
        let message_id = self.message_id.clone();
        svc.sender.send(Message::Email(self)).await.map_err(|e| {
            warn!("Failed to send message: {:?}", e);
            Status::internal("Failed to send message")
        })?;
        Ok(SendResponse {
            message_id,
            timestamp: Some(to_ts()),
        })
    }
}

impl From<EmailMessage> for Message {
    fn from(email: EmailMessage) -> Self {
        Self::Email(email)
    }
}

impl From<EmailMessage> for SendRequest {
    fn from(email: EmailMessage) -> Self {
        Self {
            message: Some(email.into()),
        }
    }
}

#[cfg(feature = "test_utils")]
impl EmailMessage {
    pub fn fake() -> Self {
        use fake::{faker::internet::en::SafeEmail, Fake};
        use uuid::Uuid;
        Self {
            message_id: Uuid::new_v4().to_string(),
            sender: SafeEmail().fake(),
            recipients: vec![SafeEmail().fake()],
            subject: "Hello".to_string(),
            body: "Hello, world!".to_string(),
        }
    }
}
