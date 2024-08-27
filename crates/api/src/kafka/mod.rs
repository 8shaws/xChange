use lazy_static::lazy_static;
use rdkafka::config::ClientConfig;
use rdkafka::error::KafkaError;
use rdkafka::message::OwnedMessage;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use std::sync::Mutex;
use std::time::Duration;

lazy_static! {
    static ref KAFKA_PRODUCER: Mutex<Option<KafkaProducer>> = Mutex::new(None);
}

pub struct KafkaProducer {
    producer: FutureProducer,
}

impl KafkaProducer {
    pub fn instance(
        brokers: &str,
    ) -> Result<&'static Mutex<Option<KafkaProducer>>, rdkafka::error::KafkaError> {
        let mut producer = KAFKA_PRODUCER.lock().unwrap();
        if producer.is_none() {
            let new_producer = KafkaProducer::new(brokers)?;
            *producer = Some(new_producer);
        }
        Ok(&KAFKA_PRODUCER)
    }

    fn new(brokers: &str) -> Result<Self, rdkafka::error::KafkaError> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()?;

        Ok(KafkaProducer { producer })
    }

    pub async fn send_message(
        &self,
        topic: &str,
        key: Option<&str>,
        payload: &str,
    ) -> Result<(i32, i64), rdkafka::error::KafkaError> {
        let mut record = FutureRecord::to(topic).payload(payload);
        if let Some(k) = key {
            record = record.key(k);
        }

        match self
            .producer
            .send(record, Timeout::After(Duration::from_secs(0)))
            .await
        {
            Ok((partition, offset)) => Ok((partition, offset)),
            Err((err, _)) => Err(err),
        }
    }

    // pub async fn send_messages(
    //     &self,
    //     topic: &str,
    //     messages: Vec<(Option<String>, String)>,
    // ) -> Vec<Result<(i32, i64), (KafkaError, OwnedMessage)>> {
    //     let futures = messages.into_iter().map(|(key, payload)| {
    //         let mut record = FutureRecord::to(topic).payload(&payload);
    //         if let Some(k) = key {
    //             record = record.key(&k as &str);
    //         }
    //         self.producer
    //             .send(record, Timeout::After(Duration::from_secs(0)))
    //     });

    //     futures::future::join_all(futures).await
    // }
}
