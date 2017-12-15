// Helper networks
pub fn convert<A, B>(val: Vec<B>) -> Vec<A> where B: Into<A>, B: Clone {
    val.iter().map(|x| x.clone().into()).collect()
}

/*****************/
/**   MARKETS   **/
/*****************/

#[derive(Debug, Clone)]
pub struct SingleMarket {
    exch_id: u32,
    exch_name: String,
    exch_code: String,
    mkt_id: u32,
    mkt_name: String,
    exchmkt_id: u32
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleMarketTemp {
    exch_id: String,
    exch_name: String,
    exch_code: String,
    mkt_id: String,
    mkt_name: String,
    exchmkt_id: String
}

impl From<SingleMarketTemp> for SingleMarket {
    fn from(val: SingleMarketTemp) -> Self {
        SingleMarket {
            exch_id: val.exch_id.parse().unwrap(),
            exch_name: val.exch_name,
            exch_code: val.exch_code,
            mkt_id: val.mkt_id.parse().unwrap(),
            mkt_name: val.mkt_name,
            exchmkt_id: val.exchmkt_id.parse().unwrap(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseMarket(pub Vec<SingleMarketTemp>);

/*****************/
/**  EXCHANGES  **/
/*****************/

#[derive(Debug, Clone)]
pub struct SingleExchange {
    pub exch_id: i32,
    pub exch_name: String,
    pub exch_code: String,
    pub exch_fee: f32,
    pub exch_trade_enabled: i32,
    pub exch_balance_enabled: i32,
    pub exch_url: String
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleExchangeTemp {
    pub exch_id: String,
    pub exch_name: String,
    pub exch_code: String,
    pub exch_fee: String,
    pub exch_trade_enabled: String,
    pub exch_balance_enabled: String,
    pub exch_url: String
}

impl From<SingleExchangeTemp> for SingleExchange {
    fn from(val: SingleExchangeTemp) -> Self {
        SingleExchange {
            exch_id: val.exch_id.parse().unwrap(),
            exch_name: val.exch_name,
            exch_code: val.exch_code,
            exch_fee: val.exch_fee.parse().unwrap(),
            exch_trade_enabled: val.exch_trade_enabled.parse().unwrap(),
            exch_balance_enabled: val.exch_balance_enabled.parse().unwrap(),
            exch_url: val.exch_url,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseExchange(pub Vec<SingleExchangeTemp>);


/*****************/
/** ORDER TYPES **/
/*****************/
#[derive(Debug, Clone)]
pub struct SingleOrderType {
    pub order_type_id: i32,
    pub name: String,
    pub order: i32
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleOrderTypeTemp {
    pub order_type_id: String,
    pub name: String,
    pub order: String
}

impl From<SingleOrderTypeTemp> for SingleOrderType {
    fn from(val: SingleOrderTypeTemp) -> Self {
        SingleOrderType {
            order_type_id: val.order_type_id.parse().unwrap(),
            name: val.name,
            order: val.order.parse().unwrap(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseOrderType(pub Vec<SingleOrderTypeTemp>);

/*****************/
/** ACCOUNTS **/
/*****************/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleAccount {
    pub auth_id: String,
    pub auth_key: String,
    pub auth_optional1: String,
    pub auth_nickname: String,
    pub exch_name: String,
    pub auth_secret: String,
    pub auth_updated: String,
    pub auth_active: String,
    pub auth_trade: String,
    pub exch_trade_enabled: String,
    pub exch_id: String
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleAccountTemp {
    pub auth_id: String,
    pub auth_key: String,
    pub auth_optional1: Option<String>,
    pub auth_nickname: String,
    pub exch_name: String,
    pub auth_secret: String,
    pub auth_updated: Option<String>,
    pub auth_active: String,
    pub auth_trade: String,
    pub exch_trade_enabled: Option<String>,
    pub exch_id: String
}


impl From<SingleAccountTemp> for SingleAccount {
    fn from(val: SingleAccountTemp) -> Self {
        SingleAccount {
            auth_id: val.auth_id,
            auth_key: val.auth_key,
            auth_optional1: match val.auth_optional1 {
                Some(x) => x,
                None => " ".to_owned()
            },
            auth_nickname: val.auth_nickname,
            exch_name: val.exch_name,
            auth_secret: val.auth_secret,
            auth_updated: match val.auth_updated {
                Some(x) => x,
                None => "2000-01-01 12:00:00".to_owned()
            },
            auth_active: val.auth_active,
            auth_trade: val.auth_trade,
            exch_trade_enabled: match val.exch_trade_enabled {
                Some(x) => x,
                None => "0".to_owned()
            },
            exch_id: val.exch_id,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseAccount(pub Vec<SingleAccount>);

/*****************/
/**   ORDERS   **/
/*****************/
#[derive(Serialize, Deserialize, Debug, Clone)]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleOrderTemp {
    pub exch_id: String,
    pub exch_name: String,
    pub mkt_name: String,
    pub auth_id: String,
    pub executed_price: String,
    pub limit_price: String,
    pub order_id: String,
    pub order_type: String,
    pub order_price_type: String,
    pub order_status: String,
    pub quantity: String,
    pub quantity_remaining: String,
    pub last_updated: String,
    pub order_time: String,
    pub auth_nickname: String,
    pub exch_code: String,
    pub display_name: String,
    pub unixtime: String
}

impl From<SingleOrderTemp> for SingleOrder {
    fn from(val: SingleOrderTemp) -> Self {
        SingleOrder {
            exch_id: val.exch_id.parse().unwrap(),
            exch_name: val.exch_name,
            mkt_name: val.mkt_name,
            auth_id: val.auth_id.parse().unwrap(),
            executed_price: val.executed_price.parse().unwrap(),
            limit_price: val.limit_price.parse().unwrap(),
            order_id: val.order_id.parse().unwrap(),
            order_type: val.order_type,
            order_price_type: val.order_price_type,
            order_status: val.order_status,
            quantity: val.quantity.parse().unwrap(),
            quantity_remaining: val.quantity_remaining.parse().unwrap(),
            last_updated: val.last_updated,
            order_time: val.order_time,
            auth_nickname: val.auth_nickname,
            exch_code: val.exch_code,
            display_name: val.display_name,
            unixtime: val.unixtime.parse().unwrap(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseOrder(pub Vec<SingleOrderTemp>);


/*****************/
/**   BALANCES   **/
/*****************/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleBalance {
    pub balance_curr_code: String,
    pub balance_amount_avail: f32,
    pub balance_amount_held: f32,
    pub balance_amount_total: f32,
    pub btc_balance: f32,
    pub last_price: f32
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleBalanceTemp {
    pub balance_curr_code: String,
    pub balance_amount_avail: String,
    pub balance_amount_held: String,
    pub balance_amount_total: String,
    pub btc_balance: String,
    pub last_price: String
}

impl From<SingleBalanceTemp> for SingleBalance {
    fn from(val: SingleBalanceTemp) -> Self {
        SingleBalance {
            balance_curr_code: val.balance_curr_code,
            balance_amount_avail: val.balance_amount_avail.parse().unwrap(),
            balance_amount_held: val.balance_amount_held.parse().unwrap(),
            balance_amount_total: val.balance_amount_total.parse().unwrap(),
            btc_balance: val.btc_balance.parse().unwrap(),
            last_price: val.last_price.parse().unwrap()
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseBalance(pub Vec<SingleBalance>);

/*****************/
/**   REFRESH BALANCE   **/
/*****************/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleRefreshBalance {
    pub balance_curr_code: String,
    pub balance_amount_avail: f32,
    pub balance_amount_held: String,
    pub balance_amount_total: f32,
    pub btc_balance: f32,
    pub last_price: f32
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleRefreshBalanceTemp {
    pub balance_curr_code: String,
    pub balance_amount_avail: String,
    pub balance_amount_held: String,
    pub balance_amount_total: String,
    pub btc_balance: String,
    pub last_price: String
}

impl From<SingleRefreshBalanceTemp> for SingleRefreshBalance {
    fn from(val: SingleRefreshBalanceTemp) -> Self {
        SingleRefreshBalance {
            balance_curr_code: val.balance_curr_code,
            balance_amount_avail: val.balance_amount_avail.parse().unwrap(),
            balance_amount_held: val.balance_amount_held,
            balance_amount_total: val.balance_amount_total.parse().unwrap(),
            btc_balance: val.btc_balance.parse().unwrap(),
            last_price: val.last_price.parse().unwrap()
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseRefreshBalance(pub Vec<SingleRefreshBalance>);
