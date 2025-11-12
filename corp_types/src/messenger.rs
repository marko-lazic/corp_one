use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Messenger {
    pub nc: async_nats::Client,
    pub js: async_nats::jetstream::Context,
}

impl Messenger {
    pub fn new(nc: async_nats::Client) -> Self {
        Self {
            nc: nc.clone(),
            js: async_nats::jetstream::new(nc),
        }
    }

    pub async fn publish<T: Serialize>(
        &self,
        subject: impl Into<String>,
        event: &T,
    ) -> anyhow::Result<()> {
        let mut buf = Vec::new();
        event.serialize(&mut rmp_serde::Serializer::new(&mut buf))?;
        let first_ack = self.js.publish(subject.into(), buf.into()).await?;
        let _second_ack = first_ack.await?;
        Ok(())
    }
}
