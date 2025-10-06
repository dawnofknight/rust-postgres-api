use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::error::{KafkaError, RDKafkaErrorCode};
use serde_json::Value;

pub fn create_producer(brokers: &str) -> Result<FutureProducer, KafkaError> {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
}

pub async fn produce_json(
    producer: &FutureProducer,
    topic: &str,
    key: Option<&str>,
    payload: &Value,
) -> Result<(), KafkaError> {
    let payload_str = serde_json::to_string(payload)
        .map_err(|_| KafkaError::MessageProduction(RDKafkaErrorCode::InvalidMessage))?;
    let record = FutureRecord::to(topic)
        .payload(&payload_str)
        .key(key.unwrap_or(""));
    // Wait for delivery status
    let _ = producer.send(record, std::time::Duration::from_secs(5)).await;
    Ok(())
}