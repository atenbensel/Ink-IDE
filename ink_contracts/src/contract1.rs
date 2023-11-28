#[ink(contract)]
pub mod counter {
    #[ink(storage)]
    pub struct Counter {
        value: u64,
    }

    impl Counter {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { value: 0 }
        }

        #[ink(message)]
        pub fn get(&self) -> u64 {
            self.value
        }

        #[ink(message)]
        pub fn increment(&mut self) {
            self.value += 1;
        }

        #[ink(message)]
        pub fn decrement(&mut self) {
            if self.value > 0 {
                self.value -= 1;
            }
        }
    }
}
