// identity.rs

pub struct QuantumIdentity {
    identity_state: String,
}

impl QuantumIdentity {
    pub fn new(initial_state: &str) -> Self {
        QuantumIdentity { identity_state: initial_state.to_string() }
    }

    pub fn update_state(&mut self, new_state: &str) {
        self.identity_state = new_state.to_string();
    }

    pub fn get_state(&self) -> &str {
        &self.identity_state
    }
}