use reqwest::blocking::Client;
use reqwest::Error;

fn main() -> Result<(), Error> {
    let req_client = Client::new();
    let req_url = "https://httpbin.org/basic-auth/foo/bar";

    let user = String::from("foo");
    let pass = Option::Some(String::from("bar"));

    let res = req_client
        .get(req_url)
        .basic_auth(user, Option::<String>::None)
        // .basic_auth(user, pass)
        .send()?;

    println!("{:#?}", *&res.headers());
    println!("body:\n{}", res.text().unwrap());
    Ok(())
}
