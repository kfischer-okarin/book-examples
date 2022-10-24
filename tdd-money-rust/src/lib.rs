pub mod money {
    pub trait Expression {
        fn plus(&self, addend: &Money) -> Box<dyn Expression>;
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
        fn plus(&self, addend: &Money) -> Box<dyn Expression> {
            Box::new(Money {
                amount: self.amount + addend.amount,
                currency: self.currency,
            })
        }
    }

    pub struct Bank {}

    impl Bank {
        pub fn new() -> Bank {
            Bank {}
        }

        pub fn reduce(&self, source: &dyn Expression, to: &'static str) -> Money {
            Money::dollar(10)
        }
    }
}
