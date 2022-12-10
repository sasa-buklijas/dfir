use chrono;
use reqwest;
use std::{fs, io};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;
use std::thread;

fn download_file_to(url: &str, to: &str) {
    let resp = reqwest::blocking::get(url).unwrap();
    let body = resp.text().unwrap();
    let mut out = File::create(to).unwrap();
    io::copy(&mut body.as_bytes(), &mut out).unwrap();
}

fn main() {
    // make dir for TESTS
    let now = chrono::offset::Local::now();
    let custom_datetime_format = now.format("%Y%m%y_%H%M%S");
    let dir_t1 = format!("{}/brnc", custom_datetime_format); // brnc is blocking reqwest new client(new client for each download)
    let _new_dir = fs::create_dir_all(&dir_t1).unwrap();  
    let dir_t2 = format!("{}/broc", custom_datetime_format); // broc is blocking reqwest one client(one client for each download)
    let _new_dir = fs::create_dir_all(&dir_t2).unwrap();  
    let dir_t3 = format!("{}/brtpf", custom_datetime_format); // brtpf is blocking reqwest thread per file (new thread for each download)
    let _new_dir = fs::create_dir_all(&dir_t3).unwrap();  

    let mut urls = Vec::new();
    let mut files_t1 = Vec::new();
    let mut files_t2 = Vec::new();
    let mut files_t3 = Vec::new();
    let input = File::open("../testdata/100files10kB.txt").unwrap();
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        let file = line.unwrap();
        let url = format!("{}/{}", "https://github.com/sasa-buklijas/dfir/blob/main/testdata/100files10kB/", file);
        urls.push(url);

        let file_path_t1 = format!("{}/{}", &dir_t1, file);     
        files_t1.push(file_path_t1);

        let file_path_t2 = format!("{}/{}", &dir_t2, file);   
        files_t2.push(file_path_t2);

        let file_path_t3 = format!("{}/{}", &dir_t3, file);     
        files_t3.push(file_path_t3);
    }    
    let urls = urls;
    let files_t1 = files_t1;
    let files_t2 = files_t2;
    let files_t3 = files_t3;

    //
    //  TEST_1
    //
    let start = Instant::now();
    for (url, file) in urls.iter().zip(files_t1.iter()) {
        download_file_to(&url, file);
    }
    let duration = start.elapsed();
    println!("Download TEST_1 took: {:?}", duration);

    //
    //  TEST_2
    //
    struct Download {
        client: reqwest::blocking::Client,
    }
    
    impl Download {
        fn new() -> Download {
            Download{ client: reqwest::blocking::Client::builder().build().unwrap(), }
        }
        
        fn download_file_to(&self, url: &str, to: &str) {
            let resp = self.client.get(url).send().unwrap();
            let body = resp.text().unwrap();
            let mut out = File::create(to).unwrap();
            io::copy(&mut body.as_bytes(), &mut out).unwrap();
        }
    }

    let start = Instant::now();
    let downloader = Download::new();
    for (url, file) in urls.iter().zip(files_t2.iter()) {
        downloader.download_file_to(&url, file);
    }
    let duration = start.elapsed();
    println!("Download TEST_2 took: {:?}", duration);

    //
    //  TEST_3
    //
    let start = Instant::now();
    let handle = thread::spawn(move || {
        for (url, file) in urls.iter().zip(files_t3.iter()) {
            download_file_to(&url, file);
        }
    });
    handle.join().unwrap();
    let duration = start.elapsed();
    println!("Download TEST_3 took: {:?}", duration);

    //
    // delete test directory and all files
    // 
    fs::remove_dir_all(custom_datetime_format.to_string()).unwrap();

}