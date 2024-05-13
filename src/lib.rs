use near_sdk::near;

#[near(contract_state)]
#[derive(Default)]
pub struct Contract {
    length: u32,
}

#[near]
impl Contract {
    pub fn add_string(&mut self, text: String) {
        self.length = text.len() as u32;
    }

    pub fn get_length(&self) -> u32 {
        self.length
    }
}