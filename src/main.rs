use dotenv::dotenv;
use reqwest::Error;
use std::{collections::HashMap, env};

#[derive(Debug)]
struct Portfolio {
    cash_balance: f32,
    assets: HashMap<String, f32>,
}

#[tokio::main]
async fn get_current_stock_price(ticker: &str) -> Result<f32, Error> {
    // Reading API KEY
    dotenv().ok();
    let api_key = match env::var("API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Error: API_KEY environment variable not found!");
            std::process::exit(1);
        }
    };

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

fn calculate_portfolio_worth(portfolio: Portfolio) -> f32 {
    let mut total: f32 = 0.0;

    for stock in portfolio.assets {
        
        let ticker = stock.0;
        let amount_shares = stock.1;
        let price_per_share = get_current_stock_price(ticker.as_str());

        match price_per_share {
            Ok(value) => {
                let share_worth = amount_shares * value;
                println!("Ticker: {}, Shares: {}, Price per share: {:?}, Total: {:?}",
                ticker,
                amount_shares,
                price_per_share.unwrap(),
                share_worth);

                total += share_worth;
            }
            Err(err) => {
                println!("An error ocurred: {}", err);
            }
        }
    }
    return total;
}

fn add_stock_to_portfolio(mut portfolio: Portfolio, symbol: String, amount_of_shares: f32) -> Portfolio {
    println!("Adding, {}: {}, to portfolio", symbol, amount_of_shares);
    portfolio.assets.insert(symbol, amount_of_shares);
    portfolio
}

fn update_cash_balance(mut portfolio: Portfolio, update_value: f32) -> Portfolio {
    let current_value = portfolio.cash_balance;
    let new_value = current_value + update_value;
    portfolio.cash_balance = new_value;
    return portfolio;
}

fn main() {

    let mut main_portfolio = Portfolio {
        cash_balance: 0.0,
        assets: HashMap::new(),
    };

    //let cost_to_buy_x_shares: f32 = price * shares;
    //println!("STOCK: {}, PRICE: ${:?}", ticker, price);
    //println!("STOCK: {}, PRICE: ${:?}, Cost to buy {} shares = {}", 
    //ticker, price, shares, cost_to_buy_x_shares);
    
    //main_portfolio.cash_balance = 100.0;
    main_portfolio = update_cash_balance(main_portfolio, 222.0);
    main_portfolio = add_stock_to_portfolio(main_portfolio, "AAPL".to_string(), 2.0);
    main_portfolio = add_stock_to_portfolio(main_portfolio, "MSFT".to_string(), 2.0);
    main_portfolio = add_stock_to_portfolio(main_portfolio, "TSLA".to_string(), 2.0);
    main_portfolio = add_stock_to_portfolio(main_portfolio, "GOOGL".to_string(), 2.0);
    main_portfolio = add_stock_to_portfolio(main_portfolio, "AMZN".to_string(), 2.0);
    //main_portfolio.assets.insert("AAPL".to_string(), 2.0);
    //main_portfolio.assets.insert("MSFT".to_string(), 1.5);
    //main_portfolio.assets.insert("GOOGL".to_string(), 3.0);
    //main_portfolio.assets.insert("AMZN".to_string(), 2.5);
    //main_portfolio.assets.insert("TSLA".to_string(), 2.0);
    dbg!(&main_portfolio);

    // let x = calculate_portfolio_worth(main_portfolio);
    // println!("Portfolio worth: ${}", x);
    
    // let ticker = "AAPL";
    // let price: f32 = 201.39;
    // let shares: f32 = 5.0;

    // if main_portfolio.cash_balance > price * shares {
    //     println!("You can buy");
    // } else {
    //     println!("You cant afford it");
    // }
}
