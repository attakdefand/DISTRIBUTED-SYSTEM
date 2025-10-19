#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_message_creation() {
        let payload = json!({"test": "data"});
        let message = crate::Message::new(
            "test_type".to_string(),
            "test_source".to_string(),
            "test_destination".to_string(),
            payload.clone(),
        );

        assert_eq!(message.message_type, "test_type");
        assert_eq!(message.source, "test_source");
        assert_eq!(message.destination, "test_destination");
        assert_eq!(message.payload, payload);
        assert!(message.correlation_id.is_none());
        assert!(message.causation_id.is_none());
    }

    #[test]
    fn test_message_with_correlation() {
        let payload = json!({"test": "data"});
        let correlation_id = uuid::Uuid::new_v4();
        let message = crate::Message::new(
            "test_type".to_string(),
            "test_source".to_string(),
            "test_destination".to_string(),
            payload,
        ).with_correlation(correlation_id);

        assert_eq!(message.correlation_id, Some(correlation_id));
    }

    #[test]
    fn test_message_with_causation() {
        let payload = json!({"test": "data"});
        let causation_id = uuid::Uuid::new_v4();
        let message = crate::Message::new(
            "test_type".to_string(),
            "test_source".to_string(),
            "test_destination".to_string(),
            payload,
        ).with_causation(causation_id);

        assert_eq!(message.causation_id, Some(causation_id));
    }

    #[test]
    fn test_message_with_header() {
        let payload = json!({"test": "data"});
        let mut headers = HashMap::new();
        headers.insert("test_key".to_string(), "test_value".to_string());

        let message = crate::Message::new(
            "test_type".to_string(),
            "test_source".to_string(),
            "test_destination".to_string(),
            payload,
        ).with_header("test_key".to_string(), "test_value".to_string());

        assert_eq!(message.headers.get("test_key"), Some(&"test_value".to_string()));
    }
}