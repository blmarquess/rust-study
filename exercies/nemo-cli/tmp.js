const currenciStruct = {
    "USDBRL": {
        "code": "USD",
        "codein": "BRL",
        "name": "Dólar Americano/Real Brasileiro",
        "high": "4.8145",
        "low": "4.8145",
        "varBid": "0",
        "pctChange": "0",
        "bid": "4.814",
        "ask": "4.815",
        "timestamp": "1703628004",
        "create_date": "2023-12-26 19:00:04"
    }
}

const api = "https://economia.awesomeapi.com.br/json/last/USD-BRL"

const task = `
Ideia de projeto de aprendisagem, criar uma CLI que verifica o valor do dolares para reais (USD -> BRL)
Essa api deve ter os seguintes comandos
[] - Sem argumentos retorna o valor do dolar em R$
[] - Args:
[] -c --convert [ recebe um valor (float64) e converte desse valor em USD para BRL]
[] -tc --to-coins [ converte da uma moeda x para y(USD-> BRL, BRL -> USD)]
[] -l --list [ lista as moedas disponiveis na api]

api disponivel free para testes [
https://economia.awesomeapi.com.br/json/last/USD-BRL,
https://www.alphavantage.co/documentation/
]
`
