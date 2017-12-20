// // /Users/davidal/documents/wolfofcrypto/src/database/sqlite_database.db");
infer_schema!("dotenv:DATABASE_URL");

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name="currency_table"]
pub struct Record {
    pub currency: String,
    pub time: i32,
    pub market_cap: f32,
    pub price_btc: f32,
    pub price_usd: f32,
    pub vol_usd: f32
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

pub type InsertionType = (
    Vec<IntRecord>
    , Vec<FloatRecord>
    , Vec<FloatRecord>
    , Vec<FloatRecord>
);

pub type CurrencySelectionTuple = (bool, bool, bool); //BTC, ETH, LTC