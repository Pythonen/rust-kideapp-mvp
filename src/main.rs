use std::io::Write;

use async_recursion::async_recursion;
use chrono::{naive, offset, DateTime, Duration, Local, NaiveDateTime, Utc};
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
    print!("Enter kide.app product url: ");
    std::io::stdout().flush().unwrap();
    let mut product_id = String::new();
    std::io::stdin().read_line(&mut product_id).unwrap();
    let time_to_sale = check_time(product_id).await;
    println!("{:?}", time_to_sale);
    let now = Local::now();
    let naive_date = NaiveDateTime::parse_from_str(&time_to_sale, "%Y-%m-%dT%H:%M:%S%z").unwrap();
    let other_dt = DateTime::<Utc>::from_utc(naive_date, Utc);
    let time_to_start = (other_dt - now.with_timezone(&Utc)) - Duration::hours(2);
    println!("{:?}", time_to_start);
    // let product_response = match get_product().await {
    //     Some(res) => {

    //     }
    //     None => {}
    // }
}

async fn check_time(product_id: String) -> String {
    let client = reqwest::Client::new();
    let product_url = format!("https://api.kide.app/api/products/{}", product_id);
    let result = client
        .get(product_url.as_str())
        .send()
        .await
        .unwrap()
        .json::<ProductResponse>()
        .await;
    let response = match result {
        Ok(res) => res,
        Err(_) => panic!(),
    };
    return response.model.product.date_sales_from;
}

// async fn reserve_product(){

// }

async fn get_product(product_id: String) -> Option<ProductResponse> {
    let client = reqwest::Client::new();
    let product_url = format!("https://api.kide.app/api/products/{}", product_id);
    let result = client
        .get(product_url.as_str())
        .send()
        .await
        .unwrap()
        .json::<ProductResponse>()
        .await;
    match result {
        Ok(res) => {
            if res.model.variants.len() == 0 {
                println!("Sales probably haven't started yet");
                return None;
            }
            return Some(res);
        }
        Err(_) => return None,
    }
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
