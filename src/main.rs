use std::io::Write;

use async_recursion::async_recursion;
use reqwest::{self, header::HeaderMap, Error};

use response_models::Token;

use crate::response_models::ProductResponse;
mod response_models;

#[tokio::main]
async fn main() {
    // let response = get_token().await;
    // let token = match response {
    //     Ok(res) => res,
    //     Err(_) => panic!("Something went wrong with getting the token :("),
    // };
    get_product().await;
}

async fn get_product() {
    print!("Enter kide.app product url: ");
    std::io::stdout().flush().unwrap();
    let mut product_id = String::new();
    std::io::stdin().read_line(&mut product_id).unwrap();
    let client = reqwest::Client::new();
    let product_url = format!("https://api.kide.app/api/products/{}", product_id);
    let result = client
        .get(product_url.as_str())
        .send()
        .await
        .unwrap()
        .json::<ProductResponse>()
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

#[async_recursion]
async fn get_token() -> Result<Token, Error> {
    let (email, password) = get_creds();
    let mut headers = HeaderMap::new();
    headers.insert(
        "User-Agent",
        "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0"
            .parse()
            .unwrap(),
    );
    let data = format!("client_id=56d9cbe22a58432b97c287eadda040df&grant_type=password&password={}&rememberMe=true&username={}", password, email);
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

    if !result.is_ok() {
        println!("Wrong email or password");
        get_token().await?;
    }

    return result;
}
