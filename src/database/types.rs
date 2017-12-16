// // /Users/davidal/documents/wolfofcrypto/src/database/sqlite_database.db");
infer_schema!("dotenv:DATABASE_URL");

#[derive(Insertable, Queryable, Debug)]
#[table_name="bitcoin"]
pub struct BTCRecord {
    pub time: i32,
    pub market_cap: f32,
    pub price_btc: f32,
    pub price_usd: f32,
    pub vol_usd: f32,
}

#[derive(Insertable, Queryable, Debug)]
#[table_name="ethereum"]
pub struct ETHRecord {
    pub time: i32,
    pub market_cap: f32,
    pub price_btc: f32,
    pub price_usd: f32,
    pub vol_usd: f32,
}

#[derive(Insertable, Queryable, Debug)]
#[table_name="litecoin"]
pub struct LTCRecord {
    pub time: i32,
    pub market_cap: f32,
    pub price_btc: f32,
    pub price_usd: f32,
    pub vol_usd: f32,
}

//TODO: Do duplicate struct definitions as long as there is a way to add multiple tables

#[derive(Serialize, Deserialize, Debug)]
pub struct FloatRecord {
    pub unixtime: i64,
    pub floatfield: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IntRecord {
    pub unixtime: i64,
    pub intfield: i64,
}

