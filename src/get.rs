
use reqwest::header;
use reqwest::blocking::Client;
use url::Url;
use crate::bar::create_progress_bar;
use std::fs::File;
use std::io::Write;

pub fn download(target: &str, quiet_mode: bool) -> Result<(), Box<dyn::std::error::Error>> {

    let url = Url::parse(target)?;

    // Initialize HTTP client and send the request
    let client = Client::new();
    let resp = client.get(url).send()?;
    println!(
        "HTTP request sent... {}",
        format!("{}", resp.status())
    );

    // Check for success
    if resp.status().is_success() {
        let headers = resp.headers().clone();

        // Extract Content-Length (if available)
        let ct_len = headers
            .get(header::CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok());

        // Extract Content-Type (fallback to "unknown" if not present)
        let ct_type = headers
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown");

        // Print headers
        match ct_len {
            Some(len) => println!("Length: {} bytes", len),
            None => println!("Length: unknown"),
        }
        println!("Type: {}", ct_type);

        // Determine the file name
        let fname = target.split('/').last().unwrap_or("downloaded_file");
        println!("Saving to: {}", fname);

        // Create a progress bar
        let bar = create_progress_bar(quiet_mode, fname, ct_len);

        // Open the file to write data
        let mut file = File::create(fname)?;

        // Stream data in chunks
        let mut downloaded: u64 = 0;
        let content = resp.bytes()?;
        file.write_all(&content)?;
        downloaded += content.len() as u64;
        bar.set_position(downloaded);

        // Finish progress bar
        bar.finish();
    } else {
        println!("HTTP request failed with status: {}", resp.status());
    }

    Ok(())

}