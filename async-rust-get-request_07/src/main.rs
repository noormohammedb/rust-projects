use error_chain::error_chain;

error_chain! {
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("https://httpbin.org/get").await.unwrap();
    println!("Status: {}", res.status());
    let res_header = res
        .headers()
        .iter()
        .map(|(k, v)| format!("  {}: {}\n", k, v.to_str().unwrap()))
        .collect::<String>();
    // .join("\n");
    println!("Headers:\n{}", res_header);
    println!("Body:\n{}", res.text().await.unwrap());

    Ok(())
}
