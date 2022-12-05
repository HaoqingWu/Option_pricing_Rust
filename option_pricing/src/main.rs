mod European;

fn main() {
    let option = European::EuropeanOption{
        V: None,
        K: 100.0,
        r: 0.04,
        vol: Some(0.2),
        q: 0.1,
        tau: 1.2,
        S: 80.0,
    };

    let observed_price = European::EuropeanOption{
        V: Some(option.call_value()),
        K: 100.0,
        r: 0.04,
        vol: None,
        q: 0.1,
        tau: 1.2,
        S: 80.0,
    };
    println!("The price of the call option is {}", option.call_value());
    println!("The delta of the option is {}", option.delta());
    println!("The vega of the option is {}", option.vega());

    print!("The implied volatility of the observed option is {}", observed_price.implied_vol());

}
