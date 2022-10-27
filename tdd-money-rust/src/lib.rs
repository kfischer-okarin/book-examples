pub mod money {
    use std::collections::HashMap;

    pub trait Expression: Clone {
        fn plus<T: Expression>(&self, addend: &T) -> Sum<Self, T> {
            Sum(self.clone(), addend.clone())
        }

        fn times(&self, factor: i32) -> Self;

        fn reduce(&self, bank: &Bank, to: &'static str) -> Money;
    }

    #[derive(Debug, PartialEq, Clone)]
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
    }

    impl Expression for Money {
        fn times(&self, factor: i32) -> Money {
            Money {
                amount: self.amount * factor,
                currency: self.currency,
            }
        }

        fn reduce(&self, bank: &Bank, to: &'static str) -> Money {
            Money {
                amount: self.amount / bank.rate(self.currency, to),
                currency: to,
            }
        }
    }

    #[derive(Clone)]
    pub struct Sum<T: Expression, U: Expression>(T, U);

    impl<T: Expression, U: Expression> Sum<T, U> {
        pub fn new(augend: T, addend: U) -> Sum<T, U> {
            Sum(augend, addend)
        }
    }

    impl<T: Expression, U: Expression> Expression for Sum<T, U> {
        fn times(&self, factor: i32) -> Sum<T, U> {
            Sum(self.0.times(factor), self.1.times(factor))
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

        pub fn reduce<T: Expression>(&self, source: &T, to: &'static str) -> Money {
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
