-- Your SQL goes here
CREATE TABLE ethereum(
        time INTEGER PRIMARY KEY NOT NULL,
        market_cap REAL NOT NULL,
        price_btc REAL NOT NULL,
        price_usd REAL NOT NULL,
        vol_usd REAL NOT NULL
);

CREATE TABLE bitcoin(
        time INTEGER PRIMARY KEY NOT NULL,
        market_cap REAL NOT NULL,
        price_btc REAL NOT NULL,
        price_usd REAL NOT NULL,
        vol_usd REAL NOT NULL
);

CREATE TABLE litecoin(
        time INTEGER PRIMARY KEY NOT NULL,
        market_cap REAL NOT NULL,
        price_btc REAL NOT NULL,
        price_usd REAL NOT NULL,
        vol_usd REAL NOT NULL
);