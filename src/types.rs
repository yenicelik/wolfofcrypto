//RESPONSE types
#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub status: i32,
    pub result: T,
    pub error: String,
}

//Market data type
#[derive(Serialize, Deserialize, Debug)]
pub struct Markets {
    pub notifications: String,
    pub data: Vec<ExchangeInfo>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExchangeInfo {
    pub exch_id: i32,
    pub exch_name: String,
    pub exch_code: String,
    pub mkt_id: i32,
    pub mkt_name: String,
    pub exchmkt_id: i32,
}


