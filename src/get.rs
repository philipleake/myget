
use reqwest::header;
use reqwest::blocking::Client;
use url::Url;
use crate::bar::create_progress_bar;
use std::fs::File;
use std::io::{Write, Read};

pub fn download(target: &str, quiet_mode: bool) -> Result<(), Box<dyn::std::error::Error>> {

    let url = Url::parse(target)?;

    let client = Client::new();
    let mut resp = client.get(url).send()?;
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
        let mut buffer = vec![0; 8192];
        let mut downloaded: u64 = 0;
        loop {
            match resp.read(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        break;
                    }
                    file.write_all(&buffer[..bytes_read])?;
                    downloaded += bytes_read as u64;
                    
                    if ct_len.is_none() {
                        bar.inc(bytes_read as u64);
                        bar.set_message(format!("{} ({} bytes)", fname, downloaded));
                    } else {
                        bar.set_position(downloaded);
                    }
                }
                Err(e) => {
                    bar.finish_with_message(format!("Download failed: {}", e));
                    return Err(e.into());
                }
            }
        }

        if ct_len.is_none() {
            bar.finish_with_message(format!("Download complete: {} ({} bytes)", fname, downloaded));
        } else {
            bar.finish_with_message(format!("Download complete: {}", fname));
        }
    } else {
        println!("HTTP request failed with status: {}", resp.status());
    }

    Ok(())

}