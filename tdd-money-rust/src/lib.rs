pub mod money {
    #[derive(Debug, PartialEq)]
    pub struct Dollar {
        amount: i32,
    }

    impl Dollar {
        pub fn new(amount: i32) -> Dollar {
            Dollar { amount }
        }

        pub fn times(&self, multiplier: i32) -> Dollar {
            Dollar {
                amount: self.amount * multiplier,
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct Franc {
        amount: i32,
    }

    impl Franc {
        pub fn new(amount: i32) -> Franc {
            Franc { amount }
        }

        pub fn times(&self, multiplier: i32) -> Franc {
            Franc {
                amount: self.amount * multiplier,
            }
        }
    }
}
