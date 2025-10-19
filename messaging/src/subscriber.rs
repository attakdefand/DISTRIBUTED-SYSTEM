use crate::{Message, MessagingError};
use async_nats::Client;
use tracing::info;
use futures::StreamExt;

pub struct Subscriber {
    client: Client,
}

impl Subscriber {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn subscribe<F, Fut>(
        &self,
        subject: &str,
        mut handler: F,
    ) -> Result<(), MessagingError>
    where
        F: Fn(Message) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<(), MessagingError>> + Send,
    {
        let subject_owned = subject.to_string();
        let mut subscriber = self.client.subscribe(subject_owned).await
            .map_err(|e| MessagingError::NatsError(Box::new(e)))?;
        info!("Subscribed to subject {}", subject);

        while let Some(message) = subscriber.next().await {
            let payload: Message = match serde_json::from_slice(&message.payload) {
                Ok(msg) => msg,
                Err(e) => {
                    tracing::error!("Failed to deserialize message: {}", e);
                    continue;
                }
            };

            if let Err(e) = handler(payload).await {
                tracing::error!("Error handling message: {:?}", e);
            }
        }

        Ok(())
    }

    pub async fn subscribe_to_events<F, Fut>(
        &self,
        event_type: &str,
        handler: F,
    ) -> Result<(), MessagingError>
    where
        F: Fn(Message) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<(), MessagingError>> + Send,
    {
        let subject = format!("events.{}", event_type);
        self.subscribe(&subject, handler).await
    }

    pub async fn subscribe_to_commands<F, Fut>(
        &self,
        command_type: &str,
        handler: F,
    ) -> Result<(), MessagingError>
    where
        F: Fn(Message) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<(), MessagingError>> + Send,
    {
        let subject = format!("commands.{}", command_type);
        self.subscribe(&subject, handler).await
    }
}