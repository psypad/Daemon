mod trail_zip;
mod file_control;
mod worker;
mod rabbit_pub;

use file_control::controller;

use std::process::Command;
use std::str;
use lapin::{
    options::*,
    types::FieldTable,
    Connection, ConnectionProperties,
};
//use futures_util::stream::StreamExt;
use tokio;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct MalwareJob {
    file_hash: String,
    job_id: i32,
}


fn ping() -> bool {
    let target = "192.168.100.20";

    // The 'ping' command arguments differ slightly by OS.
    // -c 4 for Linux/macOS means send 4 packets
    // -n 4 for Windows means send 4 packets
    let ping_command = if cfg!(target_os = "linux") {
        Command::new("ping")
            .arg("-c")
            .arg("4")
            .arg(target)
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("ping")
            .arg("-c")
            .arg("4")
            .arg(target)
            .output()
            .expect("failed to execute process")
    };

    if ping_command.status.success() {
        let stdout = str::from_utf8(&ping_command.stdout).unwrap();
        println!("Ping successful:\n{}", stdout);
        return true
        // You can parse 'stdout' here to extract statistics like min, max, avg times.
    } else {
        let stderr = str::from_utf8(&ping_command.stderr).unwrap();
        eprintln!("Ping failed:\n{}", stderr);
        return false
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let addr = "amqp://guest:guest@192.168.100.10:5672/%2f";

    let conn = Connection::connect(
        addr,
        ConnectionProperties::default(), 
    ).await?;

    let channel = conn.create_channel().await?;

    channel
        .queue_declare(
            "Malware process queue".into(),  
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!("Waiting for jobs...");

    loop {
        let q = channel
            .queue_declare(
                "Malware process queue".into(),
                QueueDeclareOptions {
                    passive: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        println!("message count {}", q.message_count());

        if q.message_count() > 0 {
            println!("Job detected. Checking system...");

            //logic has to be changes here to allow for processing to run sleep() for x seconds maybe
            if ping() {
                if let Some(delivery) = channel
                    .basic_get(
                        "Malware process queue".into(),
                        BasicGetOptions::default(),
                    )
                    .await?
                {
                    match serde_json::from_slice::<MalwareJob>(&delivery.data) {
                        Ok(job) => {
                            println!("Processing job {}", job.job_id);

                            let malware_hash = job.file_hash;
                            let job_id = job.job_id;
                            controller(&malware_hash, job_id);

                            // process job here

                            channel
                                .basic_ack(
                                    delivery.delivery_tag,
                                    BasicAckOptions::default(),
                                )
                                .await?;
                        }
                        Err(e) => {
                            println!("Invalid job: {}", e);
                        }
                    }
                }
            } else {
                println!("System busy, waiting...");
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }


    //Ok(())
}
