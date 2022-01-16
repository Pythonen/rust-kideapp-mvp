use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Token {
    pub access_token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProductResponse {
    pub model: Product,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Product {
    pub variants: Vec<Variant>,
    pub product: ProductInfo,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ProductInfo {
    #[serde(rename(deserialize = "dateSalesFrom"))]
    pub date_sales_from: String,
    #[serde(rename(deserialize = "minTotalReservationsPerCheckout"))]
    pub min_total_reservations_per_checkout: Option<i32>,
    #[serde(rename(deserialize = "maxTotalReservationsPerCheckout"))]
    pub max_total_reservatoions_per_checkout: Option<i32>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Variant {
    #[serde(rename(deserialize = "inventoryId"))]
    pub inventory_id: String,
    #[serde(rename(deserialize = "productVariantMaximumReservableQuantity"))]
    pub product_variant_maximum_reservable_quantity: i32,
    #[serde(rename(deserialize = "isProductVariantVisible"))]
    pub is_product_variant_visible: bool,
    #[serde(rename(deserialize = "isProductVariantMarkedAsOutOfStock"))]
    pub is_product_variant_marked_as_out_of_stock: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TicketReserveInfo {
    #[serde(rename(deserialize = "inventoryId"))]
    pub inventory_id: String,
    #[serde(rename(deserialize = "productVariantUserForm"))]
    pub product_variant_user_form: Option<String>,
    #[serde(rename(deserialize = "quantity"))]
    pub quantity: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReserveInfo {
    #[serde(rename(deserialize = "toCreate"))]
    pub to_create: Vec<TicketReserveInfo>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReservationResponse {
    pub model: ReservationModelResponse,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReservationModelResponse {
    #[serde(rename(deserialize = "finalPrice"))]
    pub final_price: i32,
    #[serde(rename(deserialize = "serviceFee"))]
    pub service_fee: i32,
    #[serde(rename(deserialize = "reservationsCount"))]
    pub reservations_count: i32,
}
