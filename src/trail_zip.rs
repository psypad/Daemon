use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

pub fn log_zipper(malware_hash: &str) -> Result<(), Box<dyn std::error::Error>> {
    let hash_value = malware_hash;

    let source_folder = "/home/omr/data_logs";
    let destination_folder = "/home/omr/report";
    let temp_zip_name = format!("{}.zip", hash_value);
    let final_zip_name = format!("{}.zip", hash_value);

    fs::create_dir_all(source_folder)?;
    fs::create_dir_all(destination_folder)?;

    let file = File::create(&temp_zip_name)?;
    let mut zip = ZipWriter::new(file);

    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let mut file_index = 1;

    for entry in fs::read_dir(source_folder)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            let ext_format = if extension.is_empty() {
                String::new()
            } else {
                format!(".{}", extension)
            };

            let name_str = format!("{}_{}{}", hash_value, file_index, ext_format);

            zip.start_file(name_str, options)?;
            let mut f = File::open(&path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;

            file_index += 1;
        }
    }

    zip.finish()?;

    let final_path = Path::new(destination_folder).join(&final_zip_name);
    fs::copy(&temp_zip_name, &final_path)?;
    fs::remove_file(&temp_zip_name)?;

    Ok(())
}