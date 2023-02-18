use reqwest::header::USER_AGENT;
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    site_admin: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let req_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );

    // let req_url = format!("https://httpbin.org/get",);

    println!("{}", req_url);

    let req_client = reqwest::Client::new();

    let res = req_client
        .get(&req_url)
        .header(USER_AGENT, "curl")
        .send()
        .await?;

    // dbg!(&res);
    // dbg!(&res.text().await.unwrap());
    // dbg!(&res.json::<User>().await.unwrap());

    let users: Vec<User> = res.json().await?;
    println!("total no. of users: {}", users.len());
    println!(
        "{}",
        users
            .into_iter()
            .map(|data| format!("{:#?}", data))
            .collect::<Vec<String>>()
            .join("\n")
    );

    Ok(())
}
