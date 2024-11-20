
use reqwest::header;
use reqwest::blocking::Client;
use url::Url;
use crate::bar::create_progress_bar;
use std::fs::File;
use std::io::Write;

pub fn download(target: &str, quiet_mode: bool) -> Result<(), Box<dyn::std::error::Error>> {

    let url = Url::parse(target)?;

    let client = Client::new();
    let resp = client.get(url).send()?;
    println!(
        "HTTP request sent... {}",
        format!("{}", resp.status())
    );

    if resp.status().is_success() {
        let headers = resp.headers().clone();

        let ct_len = headers
            .get(header::CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok());

        let ct_type = headers
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("unknown");

        match ct_len {
            Some(len) => println!("Length: {} bytes", len),
            None => println!("Length: unknown"),
        }
        println!("Type: {}", ct_type);

        let fname = target.split('/').last().unwrap_or("downloaded_file");
        println!("Saving to: {}", fname);

        let bar = create_progress_bar(quiet_mode, fname, ct_len);

        let mut file = File::create(fname)?;

        let mut downloaded: u64 = 0;
        let content = resp.bytes()?;
        file.write_all(&content)?;
        downloaded += content.len() as u64;
        bar.set_position(downloaded);

        bar.finish();
    } else {
        println!("HTTP request failed with status: {}", resp.status());
    }

    Ok(())

}