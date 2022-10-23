pub mod money {
    #[derive(Debug)]
    pub struct Dollar {
        pub amount: i32,
    }

    impl Dollar {
        pub fn new(amount: i32) -> Dollar {
            Dollar { amount }
        }

        pub fn times(&mut self, multiplier: i32) {}
    }
}
