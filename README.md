# Option calculator

A simple option calculator based on the Black-Scholes model.

The current version provides the following methods.
- Price European call/put options.
- Compute the Greeks, e.g., Delta, Gamma, Theta, Vega and Rho, of call/put options.
- Compute the implied volatility.

## Example
We define an option struct to define store the parameters of an option.
Note that we set option price `V` and volatility `vol` as Option enums.
```rust
let option = European::EuropeanOption{
        V: None,
        K: 100.0,
        r: 0.04,
        vol: Some(0.2),
        q: 0.1,
        tau: 1.2,
        S: 80.0,
    };
```

Compute the call option price.
```rust
option.call_value()
```

Compute the delta of the call option.
```rust
option.delta()
```

To compute the implied volatility, we can read option price from the market and set `vol` to be `None`.
In this example, we set the price as that from the previous example.
```rust
let observed_price = European::EuropeanOption{
        V: Some(option.call_value()),
        K: 100.0,
        r: 0.04,
        vol: None,
        q: 0.1,
        tau: 1.2,
        S: 80.0,
    };
```

Compute the implied volatility using Newton-Raphson.
```rust
observed_price.implied_vol()
```


