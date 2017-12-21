pub type CurrencySelectionTuple = (bool, bool, bool); //BTC, ETH, LTC

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