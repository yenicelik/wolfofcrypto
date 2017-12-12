fn convert<A, B>(val: Vec<B>) -> Vec<A> where B: Into<A>, B: Clone {
    val.iter().map(|x| x.clone().into()).collect()
}

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
    pub data: Vec<SingleMarket>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleMarket {
    pub exch_id: i32,
    pub exch_name: String,
    pub exch_code: String,
    pub mkt_id: i32,
    pub mkt_name: String,
    pub exchmkt_id: i32,
}


//Exchanges data type
#[derive(Serialize, Deserialize, Debug)]
pub struct Exchanges {
    pub notifications: String,
    pub data: Vec<SingleExchange>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleExchange {
    pub exch_id: i32,
    pub exch_name: String,
    pub exch_code: String,
    pub exch_fee: f32,
    pub exch_trade_enabled: i32,
    pub exch_balance_enabled: i32,
    pub exch_url: String
}

//OrderTypes
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderTypes {
    pub notifications: String,
    pub data: Vec<SingleOrderType>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleOrderType {
    pub order_type_id: i32,
    pub name: String,
    pub order: i32
}

//Accounts
#[derive(Serialize, Deserialize, Debug)]
pub struct Accounts {
    pub notifications: String,
    pub data: Vec<SingleAccount>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleAccount {
    pub auth_id: i32,
    pub auth_key: String,
    pub auth_optional1: String,
    pub auth_nickname: String,
    pub exch_name: String,
    pub auth_secret: String,
    pub auth_updated: String,
    pub auth_active: i32,
    pub auth_trade: i32,
    pub exch_trade_enabled: i32,
    pub exch_id: i32
}

//Orders
#[derive(Serialize, Deserialize, Debug)]
pub struct Orders {
    pub notifications: String,
    pub data: Vec<SingleOrder>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleOrder {
    pub exch_id: i32,
    pub exch_name: String,
    pub mkt_name: String,
    pub auth_id: i32,
    pub executed_price: f32,
    pub limit_price: f32,
    pub order_id: i32,
    pub order_type: String,
    pub order_price_type: String,
    pub order_status: String,
    pub quantity: f32,
    pub quantity_remaining: f32,
    pub last_updated: String,
    pub order_time: String,
    pub auth_nickname: String,
    pub exch_code: String,
    pub display_name: String,
    pub unixtime: i32
}

