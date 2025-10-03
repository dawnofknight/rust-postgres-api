# Scrape Consumer

Consumes crawl results from Kafka and inserts into Cassandra.

## Environment

- `KAFKA_BROKERS` (e.g., `broker:9092`)
- `KAFKA_TOPIC_CRAWL` (e.g., `crawl_results`)
- `CASSANDRA_CONTACT_POINTS` (e.g., `cassandra`)
- `CASSANDRA_KEYSPACE` (e.g., `scraper`)

## Run

Docker Compose defines `broker`, `cassandra`, and `consumer`. Start via root compose.