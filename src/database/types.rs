use coinmarketcap;

//TODO: Do duplicate struct definitions as long as there is a way to add multiple tables
pub type InsertionType = (
    Vec<coinmarketcap::types::IntRecord>
    , Vec<coinmarketcap::types::FloatRecord>
    , Vec<coinmarketcap::types::FloatRecord>
    , Vec<coinmarketcap::types::FloatRecord>
);

