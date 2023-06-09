use std::fs::{self, File};
use std::io::Error;

use sha2::{Digest, Sha256};
use std::path::Path;



const DBDIR: &'static str = "./database";

pub fn create_database() {
    let dbpath: &str = DBDIR;

    if !(Path::new(dbpath).exists()) {
        fs::create_dir(dbpath).expect("couldnt create <dir> DBDIR");
    } else {
        println!("WARN: <DBDIR> already exists");
    }

    if !(Path::new(&(dbpath.to_owned() + "/data")).exists()) {
        fs::create_dir(dbpath.to_owned() + "/data").expect("couldnt create <dir> data");
    } else {
        println!("WARN: <DBDIR/DATA> already exists");
    }
    if !(Path::new(&(dbpath.to_owned() + "/index.txt")).exists()) {
        File::create(dbpath.to_owned() + "/index.txt").expect("couldnt create index.txt");
    } else {
        println!("WARN: <DBDIR/index.txt> file already exists");
    }
}

pub fn get_video(hash: String) -> Result<Vec<u8>, Error> {
    
    let db_path = format!("{}/data/{}", DBDIR, hash);
    println!("[INFO] Fetching data at: {}", db_path);

    let video_data = fs::read(db_path).expect("couldnt read");

    Ok(video_data)
}

pub fn add_video(video_data: &[u8]) -> Result<String, Error> {
    // // Compress the video data using the VP9 codec
    // let mut child = Command::new("ffmpeg")
    //     .args(&[
    //         "-i",
    //         "pipe:0",
    //         "-c:v",
    //         "vp9",
    //         "-b:v",
    //         "1M",
    //         "-c:a",
    //         "libopus",
    //         "-f",
    //         "webm",
    //         "pipe:1",
    //     ])
    //     .stdin(Stdio::piped())
    //     .stdout(Stdio::piped())
    //     .spawn()?;
    // let stdin = child.stdin.as_mut().unwrap();
    // stdin.write_all(video_data)?;
    // let output = child.wait_with_output()?;

    // Read the compressed video data
    println!("[INFO] reading data..");
    let compressed_data = video_data;
    

    // Generate a SHA-256 hash of the compressed video data
    println!("[INFO] generating hash..");
    let mut hasher = Sha256::new();
    hasher.update(&compressed_data);
    let hash = format!("{:x}", hasher.finalize());
        
    // Store the compressed video data in the database using the hash as the filename
    let db_path = format!("{}/data/{}", DBDIR, hash);
    fs::write(db_path.clone(), compressed_data)?;
    println!("[INFO] <SUCCESS> file saved at: {}", db_path);    
    // Return the hash as the video's address
    Ok(hash)
}
