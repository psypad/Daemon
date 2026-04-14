
//use std::io::prelude::*;
use std::net::TcpStream;
use ssh2::Session;
//use std::path::Path;
//use std::process::{Command, Child};
use std::io::Read;


pub fn ssher(hash_value: &str) -> Result<(), Box<dyn std::error::Error>> {

    // let mut tshark: Child = Command::new("tshark")
    // .args(&[
    //     "-i", "eth0",                 // interface
    //     "-f", "host 10.10.10.2 and not port 22",
    //     "-w", &format!("captures/{}.pcap", hash_value)
    // ])
    // .spawn()?; 

    println!("tshark started...");

    let tcp = TcpStream::connect("10.10.10.2:22")?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    sess.userauth_password("jugaad_profiler", "secret@123")?;

    if !sess.authenticated() {
        return Err("Authentication failed".into());
    }

    let mut channel = sess.channel_session()?;


    //--------------------------------- Command that is executed in the malware machine --------------------------------

    println!("ssh successful, running scripts...");

    let command = format!(
        r#"powershell -ExecutionPolicy Bypass -File "C:\Users\jugaad_profiler\Documents\Command_center\detonator.ps1" -HashValue "{}""#,
        hash_value
    );

    println!("script ran");

    channel.exec(&command)?;

    println!("now printing powershell output");

    // reading the output from the windows power shell instance

    let mut output = String::new();
    channel.read_to_string(&mut output)?;
    println!("{}", output);

    //closing the connection

    channel.wait_close()?;
    println!("Exit Status: {}", channel.exit_status()?);

    // tshark.kill()?;       // terminate capture
    // tshark.wait()?;       

    println!("tshark stopped.");

    Ok(())
}