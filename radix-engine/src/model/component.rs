use sbor::*;
use scrypto::types::*;

#[derive(Debug, Clone, Encode, Decode)]
pub struct Component {
    blueprint: Address,
    name: String,
    state: Vec<u8>,
}

impl Component {
    pub fn new(blueprint: Address, name: String, state: Vec<u8>) -> Self {
        Self {
            blueprint,
            name,
            state,
        }
    }

    pub fn blueprint(&self) -> Address {
        self.blueprint
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn state(&self) -> &[u8] {
        &self.state
    }

    pub fn set_state(&mut self, new_state: Vec<u8>) {
        self.state = new_state;
    }
}