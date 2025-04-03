use std::thread;
use std::io::prelude::*;
use serde_json;
use solutions::TransformAndPossibilitiesList;
mod interface;
mod solutions;
mod histogram;
mod words;
//go find george to toy ____ nam?


fn main() {
    //by default we'll check output.json.zstd for a prior output run
    let path = match std::env::args().nth(1) {
        Some(input_path) => input_path,
        None => "output.json.zstd".to_string(),
    };
    
    let result = match read_compressed_file(&path) {
        // if we already have an output file then life is good
        Ok(output) => {
            println!("Prior solutions list found at {}", path);
            output
        },
        // Otherwise Calculate one and write it to file
        Err(_) => {
            println!("No prior solutions file found at {}\n Will now begin calculating solution...", path);
            thread::sleep(std::time::Duration::from_secs(1));
            let output = solutions::get_all_solutions();
            write_compressed_file(&"output.json.zstd".to_string(), &output);
            output
        }
    };

    interface::run(result);    
}

fn read_compressed_file(path: &String) -> Result<TransformAndPossibilitiesList,Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    println!("File opened");
    let mut decompressed = String::new();
    let mut writer = zstd::stream::Decoder::new(file)?;
    println!("decompressor opened");
    writer.read_to_string(&mut decompressed)?;

    println!("File read");
    let result = serde_json::from_str(
        decompressed.as_str()
    )?;

    println!("Processed and loaded {}",path);
    Ok(result)
}

fn write_compressed_file(path: &String, filedata: &TransformAndPossibilitiesList) {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap();

    let serialized = serde_json::to_string(filedata).unwrap();

    let encoded = zstd::encode_all(serialized.as_bytes(), 0).unwrap();

    file.write_all(&encoded).unwrap();

}
