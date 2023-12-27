use clap::Parser;
use reqwest;
use serde_derive::{Deserialize, Serialize};
use serde_json;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// nome do individo!!
    #[arg(short, long, default_value = " ")]
    name: String,

    /// converte os valores de uma moeda para outra
    #[arg(short, long, default_value = "USDBRL")]
    convert: String,

    /// valor para input de conversão
    #[arg(short, long, default_value = "1.0")]
    value: f64,
}

#[derive(Debug, Deserialize)]
#[warn(dead_code)]
struct CurrencyData {
    code: String,
    codein: String,
    name: String,
    high: String,
    low: String,
    var_bid: String,
    pct_change: String,
    bid: String,
    ask: String,
    timestamp: String,
    create_date: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    #[serde(rename = "USDBRL")]
    usd_brl: CurrencyData,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let currency_api_response = client
        .get("https://economia.awesomeapi.com.br/json/last/USD-BRL")
        .send()
        .await?
        .text()
        .await?;
    let response_json: ApiResponse = serde_json::from_str(&currency_api_response).unwrap();
    let args = Args::parse();
    println!("Hello, world {}!", args.name);
    print!("{:?}", response_json.usd_brl);
    Ok(())
}

// Ideia de projeto de aprendisagem, criar uma CLI que verifica o valor do dolares para reais (USD -> BRL)
// Essa api deve ter os seguintes comandos
// [] - Sem argumentos retorna o valor do dolar em R$
// [] - Args:
// [] -c --convert [ recebe um valor (float64) e converte desse valor em USD para BRL]
// [] -tc --to-coins [ converte da uma moeda x para y(USD-> BRL, BRL -> USD)]
// [] -l --list [ lista as moedas disponiveis na api]
//
// api disponivel free para testes [
// https://economia.awesomeapi.com.br/json/last/USD-BRL,
// https://www.alphavantage.co/documentation/
// ]
