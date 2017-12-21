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
    pub currency: Option<String> //If none is provided, get all currency pairs
}