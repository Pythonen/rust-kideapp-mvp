use std::{collections::HashMap, io::Write, thread};

use async_recursion::async_recursion;
use chrono::{DateTime, Duration, Local, NaiveDateTime, Utc};
use reqwest::{self, header::HeaderMap, Error};

use response_models::{TicketReserveInfo, Token};

use crate::response_models::ProductResponse;
mod response_models;

#[tokio::main]
async fn main() {
    let response = get_token().await;
    let token = match response {
        Ok(res) => res.access_token,
        Err(_) => panic!("Something went wrong with getting the token :("),
    };
    print!("Enter kide.app product url: ");
    std::io::stdout().flush().unwrap();
    let mut product_id = String::new();
    std::io::stdin().read_line(&mut product_id).unwrap();

    let time_to_sale = check_time(&product_id).await;
    let now = Local::now();
    let naive_date = NaiveDateTime::parse_from_str(&time_to_sale, "%Y-%m-%dT%H:%M:%S%z").unwrap();
    let other_dt = DateTime::<Utc>::from_utc(naive_date, Utc);
    let time_to_start = (other_dt - now.with_timezone(&Utc)) - Duration::hours(2);
    if time_to_start > Duration::seconds(0) {
        thread::sleep(time_to_start.to_std().unwrap());
    }

    let reserved = get_and_reserve(&product_id, &token).await;
    println!("{:?}", reserved);
}

fn variant_quantity(quantity: i32, max_variant: i32) -> i32 {
    if quantity > max_variant {
        return max_variant;
    };
    return quantity;
}

fn total_quantity(
    variation_count: i32,
    quantity: i32,
    min_total: Option<i32>,
    max_total: Option<i32>,
) -> i32 {
    if min_total.is_some() && (min_total.unwrap_or(1) / variation_count) > quantity {
        return min_total.unwrap_or(1) / variation_count;
    }
    if max_total.is_some() && quantity > (max_total.unwrap() / variation_count) {
        return max_total.unwrap_or(1);
    }
    return quantity;
}

#[async_recursion]
async fn get_and_reserve(product_id: &String, token: &String) -> String {
    let tickets = get_product(&product_id).await;
    println!("{:?}", tickets);
    if tickets.is_none() {
        get_and_reserve(&product_id, &token).await;
    }
    let (min, max) = get_min_max(product_id).await;
    let quantity = total_quantity(
        tickets
            .as_ref()
            .unwrap()
            .model
            .variants
            .len()
            .try_into()
            .unwrap(),
        5,
        min,
        max,
    );
    let invs: Vec<TicketReserveInfo> = tickets
        .unwrap()
        .model
        .variants
        .iter()
        .map(|variant| {
            return TicketReserveInfo {
                inventory_id: variant.inventory_id.clone(),
                product_variant_user_form: None,
                quantity: variant_quantity(quantity, 5).to_string(),
            };
        })
        .collect();

    return reserve_tickets(&token, &invs).await;
}

async fn reserve_tickets(token: &String, data: &Vec<TicketReserveInfo>) -> String {
    let client = reqwest::Client::new();
    let mut body = HashMap::new();
    body.insert("toCreate", data);
    let mut headers = HeaderMap::new();
    headers.insert(
        "User-Agent",
        "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:78.0) Gecko/20100101 Firefox/78.0"
            .parse()
            .unwrap(),
    );
    headers.insert("Authorization", token.parse().unwrap());
    println!("Body -> {:?}", body);
    let result = client
        .post("https://api.kide.app/api/reservations/batched")
        .headers(headers)
        .json(&body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{:?}", result);
    return result;
}

async fn check_time(product_id: &String) -> String {
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

async fn get_product(product_id: &String) -> Option<ProductResponse> {
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

async fn get_min_max(product_id: &String) -> (Option<i32>, Option<i32>) {
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
        Ok(res) => (
            res.model.product.min_total_reservations_per_checkout,
            res.model.product.max_total_reservations_per_checkout,
        ),
        Err(_) => return (Some(1), Some(1)),
    }
}
