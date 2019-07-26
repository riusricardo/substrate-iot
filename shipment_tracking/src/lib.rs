#![cfg_attr(not(any(test, feature = "std")), no_std)]

use ink_core::{
    memory::format,
    memory::string::String,
    storage,
};
use ink_lang::contract;


contract! {
    #![env = ink_core::env::DefaultSrmlTypes]

    struct ShipmentTracking {
        shipped: storage::Value<bool>,
        location: storage::Value<String>,
        temperature: storage::Value<u32>,
        time: storage::Value<u64>
    }

    impl Deploy for ShipmentTracking {
        fn deploy(&mut self) {
            self.shipped.set(false);
            self.location.set(String::from("EARTH"));
            self.temperature.set(0);

        }
    }

    impl ShipmentTracking {
        /// Flips the current state of our smart contract.
        pub(external) fn flip(&mut self) {
            *self.shipped = !*self.shipped;
        }

        /// Returns the current state.
        pub(external) fn get(&self) -> bool {
            env.println(&format!("Storage Value: {:?}", *self.shipped));
            *self.shipped
        }
    }
}