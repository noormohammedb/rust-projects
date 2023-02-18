use error_chain::error_chain;
use reqwest;
use std::fs::File;
use std::io::copy;
use tempfile::Builder;

error_chain! {
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let temp_dir = Builder::new().prefix("example").tempdir()?;
    let target_url = String::from("https://prev.rust-lang.org/logos/rust-logo-512x512.png");

    let req_client = reqwest::Client::new();

    let res = req_client.get(target_url).send().await?;

    let mut destination = {
        let file_name = res
            .url()
            .path_segments()
            .and_then(|seg| seg.last())
            .ok_or("tmp.bin")?;

        println!("file_name: {}", file_name);

        let file_name_with_temp_path = temp_dir.path().join(file_name);
        println!("location: {:?}", file_name_with_temp_path);
        File::create(file_name_with_temp_path)?
    };

    match copy(&mut res.text().await?.as_bytes(), &mut destination) {
        Ok(_) => println!("File downloaded successfully"),
        Err(e) => println!("Error: {}", e),
    };

    Ok(())
}
