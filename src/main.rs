use dotenv::dotenv;
use reqwest::Error;
use std::{thread, time};
use std::{collections::HashMap, env};

#[derive(Debug)]
struct Portfolio {
    cash_balance: f32,
    assets: HashMap<String, f32>,
}

#[derive(Debug)]
struct Trade_Position {
    ticker: String,
    open_price: f32,
    close_price: f32,
    // time_open: ?, 
    // time_close: ?,
}

#[tokio::main]
async fn finnhub_get_current_stock_price(ticker: &str) -> Result<f32, Error> {
    // Reading API KEY
    dotenv().ok();
    let api_key = match env::var("FINHUB_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Error: API_KEY environment variable not found!");
            std::process::exit(1);
        }
    };

    let url = "https://finnhub.io/api/v1/quote?symbol="
        .to_owned()
        + ticker
        + "&token="
        + &api_key;

    let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;
    // println!("{:#?}", response["c"]);
    let price = response["c"].to_string();
    //let float_price: f32 = price.parse().unwrap();
    let temp: Vec<char> = price.chars().collect();

    // temp.remove(0);
    // temp.pop();

    let temp_string: String = temp.iter().collect();
    let temp_f32: f32 = temp_string.parse().expect("Failed to parse f32");  

    dbg!(temp_f32);
    Ok(temp_f32)
    
}

// DEAD
#[tokio::main]
async fn get_current_stock_price(ticker: &str) -> Result<f32, Error> {
    // Reading API KEY
    dotenv().ok();
    let api_key = match env::var("ALPHA_API_KEY") {
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
        // let price_per_share = get_current_stock_price(ticker.as_str());
        let price_per_share = finnhub_get_current_stock_price(ticker.as_str());

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

fn remove_stock_from_portfolio(mut portfolio: Portfolio, symbol: String) -> Portfolio {
    println!("Removing, {} from portfolio", symbol);
    portfolio.assets.remove(&symbol);
    portfolio
}

fn update_cash_balance(mut portfolio: Portfolio, update_value: f32) -> Portfolio {
    let current_value = portfolio.cash_balance;
    let new_value = current_value + update_value;
    portfolio.cash_balance = new_value;
    return portfolio;
}

fn update_stock_position(mut portfolio: Portfolio, symbol: String, update_value: f32) -> Portfolio {
    
    match portfolio.assets.get_mut(&symbol) {
        Some(value) => {
            let new_value = *value + update_value;
            *value = new_value
        }
        None => {
            println!("None")
        }
    }

    portfolio
}

fn main() {

    let mut main_portfolio = Portfolio {
        cash_balance: 0.0,
        assets: HashMap::new(),
    };

    // let cost_to_buy_x_shares: f32 = price * shares;
    // println!("STOCK: {}, PRICE: ${:?}", ticker, price);
    // println!("STOCK: {}, PRICE: ${:?}, Cost to buy {} shares = {}", 
    // ticker, price, shares, cost_to_buy_x_shares);
    
    // main_portfolio.cash_balance = 100.0;
    main_portfolio = update_cash_balance(main_portfolio, 100.0);
    main_portfolio = update_cash_balance(main_portfolio, -120.0);
    main_portfolio = add_stock_to_portfolio(main_portfolio, "AAPL".to_string(), 2.0);
    main_portfolio = remove_stock_from_portfolio(main_portfolio, "AAPL".to_string());
    main_portfolio = add_stock_to_portfolio(main_portfolio, "MSFT".to_string(), 1.0);
    main_portfolio = add_stock_to_portfolio(main_portfolio, "GOOGL".to_string(), 2.0);
    main_portfolio = update_stock_position(main_portfolio, "GOOGL".to_string(), -2.0);
    main_portfolio = add_stock_to_portfolio(main_portfolio, "TSLA".to_string(), 2.0);
    main_portfolio = add_stock_to_portfolio(main_portfolio, "AMZN".to_string(), 2.0);
    main_portfolio = update_stock_position(main_portfolio, "GOOGL".to_string(), -1.0);
    // main_portfolio.assets.insert("AAPL".to_string(), 2.0);
    // main_portfolio.assets.insert("MSFT".to_string(), 1.5);
    // main_portfolio.assets.insert("GOOGL".to_string(), 3.0);
    // main_portfolio.assets.insert("AMZN".to_string(), 2.5);
    // main_portfolio.assets.insert("TSLA".to_string(), 2.0);
    
    dbg!(&main_portfolio);
    // dbg!(calculate_portfolio_worth(main_portfolio));

    let temp = Trade_Position {
        ticker: "AAPL".to_string(),
        open_price: -100000000.0,
        close_price: -100000000.0,
    };
    dbg!(temp);

    //let x = calculate_portfolio_worth(main_portfolio);
    // println!("Portfolio worth: ${}", x);

    // loop {
    //     let second = time::Duration::from_millis(1000);
    //     thread::sleep(second);
        
    //     let ticker = "TSLA";
    //     let x = finnhub_get_current_stock_price(ticker);
        
    //     println!("{}: ${:?}", ticker, x);
    // }

    // let ticker = "AAPL";
    // let price: f32 = 201.39;
    // let shares: f32 = 5.0;

    // if main_portfolio.cash_balance > price * shares {
    //     println!("You can buy");
    // } else {
    //     println!("You cant afford it");
    // }
}
