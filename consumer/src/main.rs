use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::config::ClientConfig;
use tokio_stream::StreamExt;
use serde_json::Value;

use cdrs_tokio::cluster::{NodeAddress, NodeTcpConfigBuilder};
use cdrs_tokio::cluster::session::{Session, TcpSessionBuilder};
use cdrs_tokio::cluster::session::SessionBuilder;
use cdrs_tokio::load_balancing::RoundRobinLoadBalancingStrategy;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let brokers = std::env::var("KAFKA_BROKERS").unwrap_or_else(|_| "broker:9092".to_string());
    let topic = std::env::var("KAFKA_TOPIC_CRAWL").unwrap_or_else(|_| "crawl_results".to_string());
    let cassandra_nodes = std::env::var("CASSANDRA_CONTACT_POINTS").unwrap_or_else(|_| "cassandra".to_string());
    let keyspace = std::env::var("CASSANDRA_KEYSPACE").unwrap_or_else(|_| "scraper".to_string());

    // Kafka consumer setup
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "scrape-consumer")
        .set("bootstrap.servers", &brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&[&topic])
        .expect("Can't subscribe to specified topic");

    // Cassandra setup
    let node_addr = format!("{}:9042", cassandra_nodes);
    let cluster_config = NodeTcpConfigBuilder::new()
        .with_contact_point(NodeAddress::Hostname(node_addr.clone()))
        .build()
        .await
        .expect("build cluster config");
    let session = TcpSessionBuilder::new(RoundRobinLoadBalancingStrategy::new(), cluster_config)
        .build()
        .await
        .expect("connect session");

    // Ensure keyspace and table exist
    let _ = session
        .query(format!("CREATE KEYSPACE IF NOT EXISTS {} WITH replication = {{ 'class': 'SimpleStrategy', 'replication_factor': '1' }};", keyspace))
        .await;
    let _ = session
        .query(format!("CREATE TABLE IF NOT EXISTS {}.crawl_results (id uuid PRIMARY KEY, payload text, created_at timestamp);", keyspace))
        .await;

    println!("Consumer running: brokers={}, topic={}, cassandra={}", brokers, topic, node_addr);

    let mut stream = consumer.stream();
    while let Some(result) = stream.next().await {
        match result {
            Ok(m) => {
                if let Some(payload) = m.payload_view::<str>() {
                    if let Ok(json_str) = payload {
                        // Insert into Cassandra using a simple query
                        let id = uuid::Uuid::new_v4();
                        let payload_escaped = json_str.replace('\'', "''");
                        let cql = format!(
                            "INSERT INTO {}.crawl_results (id, payload, created_at) VALUES ({}, '{}', toTimestamp(now()));",
                            keyspace,
                            id,
                            payload_escaped
                        );
                        let _ = session.query(cql).await;
                    }
                }
            }
            Err(e) => eprintln!("Kafka error: {}", e),
        }
    }
}