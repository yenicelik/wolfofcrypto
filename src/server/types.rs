//RESPONSE types
#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub status: i32,
    pub result: T,
    pub error: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostHistoricalData {
    pub start_unixtime: i32,
    pub end_unixtime: i32,
    pub currency: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub currency: String,
    pub time: i32,
    pub market_cap: f32,
    pub price_btc: f32,
    pub price_usd: f32,
    pub vol_usd: f32
}

