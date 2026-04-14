use crate::worker::ssher;
use crate::trail_zip::log_zipper;

use std::fs;
use std::path::Path;

pub fn controller(malware_hash: &str,job_id: i32) {
    let filehash_path = "/home/omr/files";
    let samba_path = "/home/omr/sample_petri_dish";
    //let processing_dir = "/home/omr/processing";


    println!("`ls /home/omr/files");

    // Construct expected file path
    let expected_file = format!("{}/{}.zip", filehash_path, malware_hash);
    let path = Path::new(&expected_file);

    if path.exists() {
        println!("Match found: {}", expected_file);
        // Do further processing here
        //------------------- moving the malware file to the samba directory ------------------

        let source_path_filehash = format!("{}/{}.zip", filehash_path, malware_hash);
        let dest_path_samba = format!("{}/{}.zip", samba_path, malware_hash);

        match fs::copy(&source_path_filehash, &dest_path_samba) {
            Ok(_) => println!("malware machine has received the file now processing...."),
            Err(e) => println!("Failed to create directory: {}", e),
        }   

        //------------------- call powershell scripts in windows --------------------------

        println!("now processing please wait....."); // processing ssh powershell scripts called here

        // sshing into the windows machine to call the scripts to start processing malware.

        match ssher(malware_hash) {
            Ok(_) => println!("completed execution"),
            Err(e) => println!("failed to activate scripts via ssh : {}", e),
        }          

        //wait for execution to finish                        

        //------------------- zip and store the files ------------------
        // fix the file names so that it corresponds to the trails exposed by the windows profiler

        // let source_path_samba_rec_os = format!("{}/{}.txt", samba_receive_path, "os-trails" );
        // let source_path_samba_rec_hard = format!("{}/{}.txt", samba_receive_path, "hardware-trails" );

        //let files:&[&str] = &[&source_path_samba_rec_hard, &source_path_samba_rec_os];


        match log_zipper(malware_hash){
            Err(why) => println!("Error reading directory: {:?}", why),
            Ok(()) => {
                        println!("logs zipped successfully!");
            }
        }

        //add the resulting zip file and jobid to the return queue

        match crate::rabbit_pub::publish(job_id, malware_hash) {
            Ok(_) => println!("Report pushed to RabbitMQ"),
            Err(e) => println!("RabbitMQ publish failed {:?}", e),
        }


    } else {
        println!("File not found: {}", expected_file);

        // Flag error, log it, push to retry queue, etc.
    }
}