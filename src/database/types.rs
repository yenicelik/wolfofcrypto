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

pub type InsertionType = (
    Vec<IntRecord>
    , Vec<FloatRecord>
    , Vec<FloatRecord>
    , Vec<FloatRecord>
);

