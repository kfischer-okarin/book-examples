pub mod money {
    #[derive(Debug, PartialEq)]
    pub enum Money<'a> {
        Dollar { amount: i32, currency: &'a str },
        Franc { amount: i32, currency: &'a str },
    }

    impl Money<'_> {
        pub fn dollar(amount: i32) -> Money<'static> {
            Money::Dollar {
                amount,
                currency: "USD",
            }
        }

        pub fn franc(amount: i32) -> Money<'static> {
            Money::Franc {
                amount,
                currency: "CHF",
            }
        }

        pub fn currency(&self) -> &str {
            match self {
                Money::Dollar { .. } => "USD",
                Money::Franc { .. } => "CHF",
            }
        }

        pub fn times(&self, multiplier: i32) -> Money {
            match self {
                Money::Dollar { amount, .. } => Money::dollar(amount * multiplier),
                Money::Franc { amount, .. } => Money::franc(amount * multiplier),
            }
        }
    }
}
