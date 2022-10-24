pub mod money {
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
}
