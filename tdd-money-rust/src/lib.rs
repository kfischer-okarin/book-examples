pub mod money {
    use std::collections::HashMap;

    pub trait Expression {
        fn plus<'a, 'b: 'a>(&'a self, addend: &'b dyn Expression) -> Box<dyn Expression + 'a>;
        fn reduce(&self, bank: &Bank, to: &'static str) -> Money;
    }

    #[derive(Debug, PartialEq)]
    pub struct Money {
        amount: i32,
        currency: &'static str,
    }

    impl Money {
        pub fn dollar(amount: i32) -> Money {
            Money {
                amount,
                currency: "USD",
            }
        }

        pub fn franc(amount: i32) -> Money {
            Money {
                amount,
                currency: "CHF",
            }
        }

        pub fn currency(&self) -> &str {
            self.currency
        }

        pub fn times(&self, multiplier: i32) -> Money {
            Money {
                amount: self.amount * multiplier,
                currency: self.currency,
            }
        }
    }

    impl Expression for Money {
        fn plus<'a, 'b: 'a>(&'a self, addend: &'b dyn Expression) -> Box<dyn Expression + 'a> {
            Box::new(Sum::new(self, addend))
        }

        fn reduce(&self, bank: &Bank, to: &'static str) -> Money {
            let rate = bank.rate(self.currency, to);
            Money {
                amount: self.amount / rate,
                currency: to,
            }
        }
    }

    pub struct Sum<'a, 'b>(&'a dyn Expression, &'b dyn Expression);

    impl Sum<'_, '_> {
        pub fn new<'a, 'b>(augend: &'a dyn Expression, addend: &'b dyn Expression) -> Sum<'a, 'b> {
            Sum(augend, addend)
        }
    }

    impl Expression for Sum<'_, '_> {
        fn plus<'a, 'b: 'a>(&'a self, _addend: &'b dyn Expression) -> Box<dyn Expression + 'a> {
            // TODO
            Box::new(Money {
                amount: 0,
                currency: "USD",
            })
        }

        fn reduce(&self, bank: &Bank, to: &'static str) -> Money {
            Money {
                amount: self.0.reduce(bank, to).amount + self.1.reduce(bank, to).amount,
                currency: to,
            }
        }
    }

    pub struct Bank {
        rates: HashMap<(String, String), i32>,
    }

    impl Bank {
        pub fn new() -> Bank {
            Bank {
                rates: HashMap::new(),
            }
        }

        pub fn reduce(&self, source: &dyn Expression, to: &'static str) -> Money {
            source.reduce(self, to)
        }

        pub fn add_rate(&mut self, from: &'static str, to: &'static str, _rate: i32) {
            self.rates
                .insert((String::from(from), String::from(to)), _rate);
        }

        pub fn rate(&self, from: &'static str, to: &'static str) -> i32 {
            if from == to {
                return 1;
            }

            self.rates[&(String::from(from), String::from(to))]
        }
    }
}
