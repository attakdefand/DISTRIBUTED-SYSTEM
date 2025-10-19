use crate::{Message, MessagingError};
use async_nats::Client;
use tracing::info;

pub struct Publisher {
    client: Client,
}

impl Publisher {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn publish(&self, subject: &str, message: Message) -> Result<(), MessagingError> {
        let payload = serde_json::to_vec(&message)?;
        let subject_owned = subject.to_string();
        self.client.publish(subject_owned, payload.into()).await
            .map_err(|e| MessagingError::NatsError(Box::new(e)))?;
        info!("Published message {} to subject {}", message.id, subject);
        Ok(())
    }

    pub async fn publish_event(&self, event_type: &str, message: Message) -> Result<(), MessagingError> {
        let subject = format!("events.{}", event_type);
        self.publish(&subject, message).await
    }

    pub async fn publish_command(&self, command_type: &str, message: Message) -> Result<(), MessagingError> {
        let subject = format!("commands.{}", command_type);
        self.publish(&subject, message).await
    }
}