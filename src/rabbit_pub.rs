use tokio::task;

pub fn publish(job_id: i32, hash: &str)
    -> Result<(), Box<dyn std::error::Error>>
{
    task::block_in_place(|| {

        let rt = tokio::runtime::Handle::current();

        rt.block_on(async {

            let conn = lapin::Connection::connect(
                "amqp://guest:guest@192.168.100.10:5672/%2f",
                lapin::ConnectionProperties::default(),
            ).await?;

            let channel = conn.create_channel().await?;

            channel.queue_declare(
                "Malware report queue".into(),
                lapin::options::QueueDeclareOptions::default(),
                lapin::types::FieldTable::default(),
            ).await?;

            let payload = serde_json::to_vec(
                &serde_json::json!({
                    "job_id": job_id,
                    "report_hash": hash
                })
            )?;

            channel.basic_publish(
                "".into(),
                "Malware report queue".into(),
                lapin::options::BasicPublishOptions::default(),
                &payload,
                lapin::BasicProperties::default(),
            ).await?
             .await?;

            Ok::<(), Box<dyn std::error::Error>>(())
        })
    })
}