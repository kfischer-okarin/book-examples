pub mod money {
    #[derive(Debug, PartialEq)]
    pub enum Money {
        Dollar { amount: i32 },
        Franc { amount: i32 },
    }

    pub struct Dollar {}

    impl Dollar {
        pub fn new(amount: i32) -> Money {
            Money::Dollar { amount }
        }
    }

    impl Money {
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

    pub struct Franc {}

    impl Franc {
        pub fn new(amount: i32) -> Money {
            Money::Franc { amount }
        }
    }
}
