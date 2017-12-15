infer_schema!("/Users/davidal/documents/wolfofcrypto/src/database/sqlite_database.db");

#[derive(Insertable, Queryable, Debug)]
#[table_name="ethereum"]
#[table_name="bitcoin"]
#[table_name="litecoin"]
pub struct Record {
    pub time: i32,
    pub market_cap: f32,
    pub price_btc: f32,
    pub price_usd: f32,
    pub vol_usd: f32,
}

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

