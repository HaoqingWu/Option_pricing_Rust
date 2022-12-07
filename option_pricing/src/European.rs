#![allow(non_snake_case)]
#![allow(dead_code)]

use statrs::distribution::{Normal, Continuous, ContinuousCDF};
use std::f64::consts::{E, PI};

use roots::find_root_newton_raphson;

pub struct EuropeanOption{
    pub V: Option<f64>, // value of the option. In practice it is observed from the market and it's used to compute the implied volatility.
    pub K: f64, // the strike price
    pub r: f64, // the risk-free rate
    pub vol: Option<f64>,  // volatility
    pub q: f64, // dividend
    pub tau: f64, // time to maturity
    pub S: f64, // price at time t, may need to be modified
}


impl EuropeanOption {
    pub fn d1(&self) -> f64{
        let sigma = match self.vol{
            Some(v) => v,
            None  => panic!("To compute d1, the volatility cannot be None!"),
        };
        let coeff = 1.0 / (sigma * (self.tau.sqrt()));
        let temp = (self.S / self.K).ln() 
                        + (self.r - self.q + sigma.powf(2.0) / 2.0) * self.tau;
        coeff * temp
    }

    pub fn delta(&self) -> f64{
        let std_normal = Normal::new(0.0, 1.0).unwrap();
        std_normal.cdf(self.d1())
    }

    pub fn d2(&self) -> f64{
        let d1 = self.d1();
        let sigma = match self.vol{
            Some(v) => v,
            None  => panic!("To compute d2, the volatility cannot be None!"),
        };

        d1 - sigma * self.tau.sqrt()
    }

    pub fn call_value(&self) -> f64{
        // Return the value of a European call option without dividend based on the Black-Scholes formula
        let std_normal = Normal::new(0.0, 1.0).unwrap();
        let call_value = self.S * std_normal.cdf(self.d1()) * E.powf(-self.q * self.tau)
                        - self.K * E.powf(-self.r * self.tau) * std_normal.cdf(self.d2());
        
        call_value
    }

    pub fn put_value(&self) -> f64{
        // Return the value of a European put option without dividend based on the Black-Scholes formula
        let std_normal = Normal::new(0.0, 1.0).unwrap();
        let put_value = self.K * E.powf(-self.r * self.tau) * std_normal.cdf(-self.d2())
                    - self.S * std_normal.cdf(-self.d1()) * E.powf(-self.q * self.tau);
        
        put_value
    }

    pub fn call_theta(&self) -> f64{
        // put option theta
        let d1 = self.d1();
        let d2 = self.d2();
        let sigma = match self.vol{
            Some(v) => v,
            None  => panic!("To compute theta, the volatility cannot be None!"),
        };
        let std_normal = Normal::new(0.0, 1.0).unwrap();
        - self.S * std_normal.pdf(d1) * sigma / (2.0 * self.tau.sqrt())
        - self.r * self.K * E.powf(-self.r * self.tau) * std_normal.cdf(d2)
    }

    pub fn put_theta(&self) -> f64{
        self.call_theta() + self.r * self.K * E.powf(-self.r * self.tau)
        
    }

    pub fn gamma(&self) -> f64{
        let sigma = match self.vol{
            Some(v) => v,
            None  => panic!("To compute gamma, the volatility cannot be None!"),
        };

        1.0 / (self.S * sigma * (2.0 * PI * self.tau).sqrt()) * E.powf(- self.d1().powf(2.0) / 2.0)
    }

    pub fn vega(&self) -> f64{
        let std_normal = Normal::new(0.0, 1.0).unwrap();
        self.S * E.powf(-self.q * self.tau) * self.tau.sqrt() * std_normal.pdf(self.d1())
    }

    pub fn call_rho(&self) -> f64{
        let std_normal = Normal::new(0.0, 1.0).unwrap();
        self.K * self.tau.sqrt() * std_normal.pdf(self.d2())
    }

    pub fn put_rho(&self) -> f64{
        let std_normal = Normal::new(0.0, 1.0).unwrap();
        -self.K * self.tau.sqrt() * std_normal.pdf(-self.d2())
    }
    
    //Newton–Raphson with ‘‘Brenner & Subrahmanyam formula’’ as initial point
    pub fn implied_vol(&self) -> f64{
        
        let val = match self.V{
            Some(v) => v,
            None  => panic!("To compute implied volatility, observed price is needed!"),
        };

        let f = |s| {
            let option = EuropeanOption{
                V: None,
                K: self.K,
                r: self.r,
                vol: Some(s),
                q: self.q,
                tau: self.tau,
                S: self.S,
            };
            val - option.call_value()
        };


        let df = |s|{
            let option = EuropeanOption{
                V: None,
                K: self.K,
                r: self.r,
                vol: Some(s),
                q: self.q,
                tau: self.tau,
                S: self.S,
            };
            - option.S * (option.tau / (2.0 * PI)).sqrt() * E.powf(-option.d1().powf(2.0) / 2.0)
        };
        // the Brenner-Subrahmanyam intial guess
        let guess: f64 = (PI / self.tau).sqrt() 
                        * (2.0 * val - self.S + E.powf(-self.r * self.tau) * self.K) 
                        / ( 2.0 * self.S);

        find_root_newton_raphson(guess, &f, &df, &mut 1e-4f64).unwrap()
    }

}
