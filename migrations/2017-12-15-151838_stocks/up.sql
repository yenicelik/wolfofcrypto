-- Your SQL goes here
CREATE TABLE currency_table(
        currency TEXT NOT NULL,
        time INTEGER NOT NULL,
        market_cap REAL NOT NULL,
        price_btc REAL NOT NULL,
        price_usd REAL NOT NULL,
        vol_usd REAL NOT NULL,
        PRIMARY KEY (time, currency)
);