pub mod money {
    #[derive(Debug, PartialEq)]
    pub enum Money {
        Dollar { amount: i32 },
        Franc { amount: i32 },
    }

    impl Money {
        pub fn dollar(amount: i32) -> Money {
            Money::Dollar { amount }
        }

        pub fn franc(amount: i32) -> Money {
            Money::Franc { amount }
        }

        pub fn times(&self, multiplier: i32) -> Money {
            match self {
                Money::Dollar { amount } => Money::Dollar {
                    amount: amount * multiplier,
                },
                Money::Franc { amount } => Money::Franc {
                    amount: amount * multiplier,
                },
            }
        }
    }
}
