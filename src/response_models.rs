use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Token {
    pub access_token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProductResponse {
    model: Product,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Product {
    variants: Vec<Variant>,
    product: ProductInfo,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ProductInfo {
    #[serde(rename(deserialize = "dateSalesFrom"))]
    date_sales_from: String,
    #[serde(rename(deserialize = "minTotalReservationsPerCheckout"))]
    min_total_reservations_per_checkout: Option<i32>,
    #[serde(rename(deserialize = "maxTotalReservationsPerCheckout"))]
    max_total_reservatoions_per_checkout: Option<i32>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Variant {
    #[serde(rename(deserialize = "inventoryId"))]
    inventory_id: String,
    #[serde(rename(deserialize = "productVariantMaximumReservableQuantity"))]
    product_variant_maximum_reservable_quantity: i32,
    #[serde(rename(deserialize = "isProductVariantVisible"))]
    is_product_variant_visible: bool,
    #[serde(rename(deserialize = "isProductVariantMarkedAsOutOfStock"))]
    is_product_variant_marked_as_out_of_stock: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TicketReserveInfo {
    #[serde(rename(deserialize = "inventoryId"))]
    inventory_id: String,
    #[serde(rename(deserialize = "productVariantUserForm"))]
    product_variant_user_form: Option<String>,
    #[serde(rename(deserialize = "quantity"))]
    quantity: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReserveInfo {
    #[serde(rename(deserialize = "toCreate"))]
    to_create: Vec<TicketReserveInfo>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReservationResponse {
    model: ReservationModelResponse,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReservationModelResponse {
    #[serde(rename(deserialize = "finalPrice"))]
    final_price: i32,
    #[serde(rename(deserialize = "serviceFee"))]
    service_fee: i32,
    #[serde(rename(deserialize = "reservationsCount"))]
    reservations_count: i32,
}
