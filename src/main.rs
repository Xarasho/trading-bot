use dotenv::dotenv;
use reqwest::Error;
use std::{collections::HashMap, env};

#[derive(Debug)]
struct Portfolio {
    cash_balance: f32,
    assets: HashMap<String, f32>,
}

#[tokio::main]
async fn get_current_stock_price(api_key: String, ticker: &str) -> Result<f32, Error> {
    

    let url = "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol="
        .to_owned()
        + ticker
        + "&apikey="
        + &api_key;

    let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    println!("{:#?}", response["Global Quote"]["05. price"]);
    let price = response["Global Quote"]["05. price"].to_string();
    //let float_price: f32 = price.parse().unwrap();
    let mut temp: Vec<char> = price.chars().collect();

    temp.remove(0);
    temp.pop();

    let temp_string: String = temp.iter().collect();
    let temp_f32: f32 = temp_string.parse().expect("Failed to parse f32");  

    dbg!(temp_f32);
    Ok(temp_f32)
}

fn main() {
    // Reading API KEY
    dotenv().ok();
    let api_key = match env::var("API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Error: API_KEY environment variable not found!");
            std::process::exit(1);
        }
    };

    
    // let ticker = "AAPL";
    // let x = get_current_stock_price(api_key, ticker).unwrap();
    // println!("STOCK: {}, PRICE: ${:?}", ticker, x);

    let mut main_portfolio = Portfolio {
        cash_balance: 0.0,
        assets: HashMap::new(),
    };

    let ticker = "AAPL";
    let price: f32 = 201.39;
    let shares: f32 = 5.0;

    let cost_to_buy_x_shares: f32 = price * shares;
    println!("STOCK: {}, PRICE: ${:?}", ticker, price);
    println!("STOCK: {}, PRICE: ${:?}, Cost to buy {} shares = {}", 
    ticker, price, shares, cost_to_buy_x_shares);

    main_portfolio.cash_balance = 100.0;
    main_portfolio.assets.insert("AAPL".to_string(), 2.0);
    main_portfolio.assets.insert("MSFT".to_string(), 1.5);
    main_portfolio.assets.insert("GOOGL".to_string(), 3.0);
    main_portfolio.assets.insert("AMZN".to_string(), 2.5);
    main_portfolio.assets.insert("TSLA".to_string(), 2.0);
    dbg!(&main_portfolio);
}
