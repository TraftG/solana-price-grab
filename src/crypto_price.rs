use rust_decimal::Decimal;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize, Debug)]
struct PriceResponse {
    solana: SolanaPrice,
}

#[derive(Deserialize, Debug)]
struct SolanaPrice {
    usd: f64,
}

pub fn get_solana_price() -> Result<Decimal, Box<dyn std::error::Error>> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=solana&vs_currencies=usd";
    let resp = reqwest::blocking::get(url)?;
    let price_response: PriceResponse = resp.json()?;

    // Конвертируем f64 в строку, затем в Decimal
    let price_str = price_response.solana.usd.to_string();
    let price = Decimal::from_str(&price_str)?;

    Ok(price)
}
