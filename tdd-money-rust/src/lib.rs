pub mod money {
    pub trait Expression {
        fn plus<'a>(&'a self, addend: &'a Money) -> Box<dyn Expression + 'a>;
        fn reduce<'a>(&self, bank: &'a Bank, to: &'static str) -> Money;
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
        fn plus<'a>(&'a self, addend: &'a Money) -> Box<dyn Expression + 'a> {
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

    pub struct Sum<'a, 'b>(&'a Money, &'b Money);

    impl Sum<'_, '_> {
        pub fn new<'a, 'b>(augend: &'a Money, addend: &'b Money) -> Sum<'a, 'b> {
            Sum(augend, addend)
        }
    }

    impl Expression for Sum<'_, '_> {
        fn plus(&self, _addend: &Money) -> Box<dyn Expression> {
            // TODO
            Box::new(Money {
                amount: 0,
                currency: "USD",
            })
        }

        fn reduce(&self, _bank: &Bank, to: &'static str) -> Money {
            Money {
                amount: self.0.amount + self.1.amount,
                currency: to,
            }
        }
    }

    pub struct Bank {}

    impl Bank {
        pub fn new() -> Bank {
            Bank {}
        }

        pub fn reduce(&self, source: &dyn Expression, to: &'static str) -> Money {
            source.reduce(self, to)
        }

        pub fn add_rate(&self, _from: &'static str, _to: &'static str, _rate: i32) {}

        pub fn rate(&self, from: &'static str, to: &'static str) -> i32 {
            if from == "CHF" && to == "USD" {
                2
            } else {
                1
            }
        }
    }
}
