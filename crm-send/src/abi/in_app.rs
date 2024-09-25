use super::{to_ts, Sender};
use crate::{
    pb::{InAppMessage, SendRequest, SendResponse},
    Message, NotificationService,
};
use tonic::Status;
use tracing::warn;

impl Sender for InAppMessage {
    async fn send(self, svc: NotificationService) -> Result<SendResponse, Status> {
        let message_id = self.message_id.clone();
        svc.sender.send(Message::InApp(self)).await.map_err(|e| {
            warn!("Failed to send message: {:?}", e);
            Status::internal("Failed to send message")
        })?;
        Ok(SendResponse {
            message_id,
            timestamp: Some(to_ts()),
        })
    }
}

#[cfg(test)]
impl InAppMessage {
    pub fn fake() -> Self {
        use uuid::Uuid;
        Self {
            message_id: Uuid::new_v4().to_string(),
            device_id: Uuid::new_v4().to_string(),
            title: "Hello".to_string(),
            body: "Hello, world!".to_string(),
        }
    }
}

impl From<InAppMessage> for Message {
    fn from(msg: InAppMessage) -> Self {
        Self::InApp(msg)
    }
}

impl From<InAppMessage> for SendRequest {
    fn from(msg: InAppMessage) -> Self {
        Self {
            message: Some(msg.into()),
        }
    }
}
