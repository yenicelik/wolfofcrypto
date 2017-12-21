
pub fn parse_values(value: Value) -> Result<(Vec<types::IntRecord>, Vec<types::FloatRecord>,
                                             Vec<types::FloatRecord>, Vec<types::FloatRecord>), Error> {
    let raw_market_cap = match value.get("market_cap_by_available_supply") {
        Some(val) => Ok(val),
        None => Err(format_err!("market_cap_by_available_supply was not found within the struct!"))
    }?;
    let market_cap: Vec<types::IntRecord> = serde_json::from_value(raw_market_cap.clone())?;

    let raw_price_btc = match value.get("price_btc") {
        Some(val) => Ok(val),
        None => Err(format_err!("price_btc was not found within the struct!"))
    }?;
    let price_btc: Vec<types::FloatRecord> = serde_json::from_value(raw_price_btc.clone())?;

    let raw_price_usd = match value.get("price_usd") {
        Some(val) => Ok(val),
        None => Err(format_err!("price_usd was not found within the struct!"))
    }?;
    let price_usd: Vec<types::FloatRecord> = serde_json::from_value(raw_price_usd.clone())?;

    let raw_vol_usd = match value.get("volume_usd") {
        Some(val) => Ok(val),
        None => Err(format_err!("volume_usd was not found within the struct!"))
    }?;
    let vol_usd: Vec<types::FloatRecord> = serde_json::from_value(raw_vol_usd.clone())?;


    Ok((market_cap, price_btc, price_usd, vol_usd))
}

//TODO: put this into an util file
pub fn str_to_currency_selection(currencies: String) -> types::CurrencySelectionTuple{
    /** The input should be comma-separated values of the names of coinmarketcap **/
    let currencies: Vec<&str> = currencies.split(", ").collect();
    println!("The currency string {:?}", currencies);

    // We want to make damn sure there's not error, so we panic if the pair is not existent
    let mut out: types::CurrencySelectionTuple = (false, false, false);

    // Test if this actually does re-assignment
    for currency in currencies {
        match currency {
            "bitcoin" => {
                out.0 = true
            },
            "ethereum" => {
                out.1 = true;
            },
            "litecoin" => {
                out.2 = true;
            },
            _ => {
                panic!("No valid currency pair is given. Because this relies on string \
                comparison, we panicked!");
            }
        };
    }

    return out;
}