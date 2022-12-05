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
    println!("The delta of the option is {}", option.delta());
    println!("The vega of the option is {}", option.vega());

}
