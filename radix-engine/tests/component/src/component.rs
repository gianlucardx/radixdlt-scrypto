use scrypto::prelude::*;

blueprint! {
    struct ComponentTest {
        test_vault: Vault,
        secret: String,
    }

    impl ComponentTest {
        fn create_test_token(amount: u32) -> Bucket {
            ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM)
                .metadata("name", "TestToken")
                .initial_supply_fungible(amount)
        }

        pub fn create_component() -> Component {
            Self {
                test_vault: Vault::with_bucket(Self::create_test_token(1000)),
                secret: "Secret".to_owned(),
            }
            .instantiate()
        }

        pub fn get_component_info(address: Address) -> Blueprint {
            Component::from(address).blueprint()
        }

        pub fn get_component_state(&self) -> String {
            self.secret.clone()
        }

        pub fn put_component_state(&mut self) -> Bucket {
            // Take resource from vault
            let bucket = self.test_vault.take(1);

            // Update state
            self.secret = "New secret".to_owned();

            bucket
        }
    }
}
