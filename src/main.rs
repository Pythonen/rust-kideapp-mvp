use std::io::Write;

use reqwest::{self, header::HeaderMap, Error};

use response_models::Token;
mod response_models;

#[tokio::main]
async fn main() {
    let (email, password) = get_creds();
    let token = get_token(email, password).await;
    match token {
        Ok(res) => println!("{:?}", res.access_token),
        Err(err) => println!("{:?}", err),
    }
    // print!("Enter kide.app product url: ");
    // std::io::stdout().flush().unwrap();
    // let mut product_id = String::new();
    // std::io::stdin().read_line(&mut product_id).unwrap();
    // get_product(product_id).await;
}

async fn get_product(product_id: String) {
    let client = reqwest::Client::new();
    let product_url = format!("https://api.kide.app/api/products/{}", product_id);
    let result = client
        .get(product_url.as_str())
        .send()
        .await
        .unwrap()
        .text()
        .await;
    println!("{:?}", result);
}

fn get_creds() -> (String, String) {
    print!("Enter kide.app email: ");
    std::io::stdout().flush().unwrap();
    let mut email = String::new();
    std::io::stdin().read_line(&mut email).unwrap();
    print!("Enter kide.app password: ");
    std::io::stdout().flush().unwrap();
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).unwrap();
    return (email.trim().to_string(), password.trim().to_string());
}

async fn get_token(email: String, password: String) -> Result<Token, Error> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "User-Agent",
        "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0"
            .parse()
            .unwrap(),
    );
    let data = format!("client_id=56d9cbe22a58432b97c287eadda040df&grant_type=password&password={}&rememberMe=true&username={}", password, email);
    println!("{:?}", data);
    let client = reqwest::Client::new();
    let result = client
        .post("https://auth.kide.app/oauth2/token")
        .body(data)
        .headers(headers)
        .send()
        .await
        .unwrap()
        .json::<Token>()
        .await;
    return result;
}
