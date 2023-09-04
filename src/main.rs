use serde::{Deserialize, Serialize};

fn main() {
    //println!("Hello, world!");
    let mut coin: String =  String::new();
    let result: Result<usize, std::io::Error> = std::io::stdin().read_line( &mut coin);
    match result {
        Ok(num_bytes) => {
            println!("Num de bytes: {num_bytes}");

            let result_precio: Result<String, ureq::Error> = get_precio(&coin);
                match result_precio {
                    Ok(precio) => println!("El precio es: {} $", precio),
                    Err(error) => println!("Ocurrió un error en la consulta del precio: {}", error),
                }
        },
        Err(error) => todo!("Ocurrió un error: {}", error),


    }
}

fn get_precio(coin: &str) -> Result<String, ureq::Error> {
    let body: String = ureq::get(&format!("https://api.coingecko.com/api/v3/coins/{}?localization=fals",coin))
        .set("Example-Header", "header value")
        .call()?
        .into_string()?;
    let coin_data: CoinData = serde_json::from_str(&body).unwrap();
    Ok(coin_data.market_data.current_price.usd.to_string())
}

#[derive(Serialize, Deserialize, Debug)]
struct CoinData{
    id: String,
    symbol: String,
    name: String,
    market_data: MarketData,
    
}
#[derive(Serialize, Deserialize, Debug)]
struct MarketData{
    current_price: Prices, 
}

#[derive(Serialize, Deserialize, Debug)]
struct Prices{
    usd: f32, 
}
