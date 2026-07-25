use canon_contracts::{OneShotOperation, Profile, StableProfileRegistry};

fn main() {
    let operation = OneShotOperation::Capabilities;
    let profiles = StableProfileRegistry::profiles();
    let change_is_stable = profiles.contains(&Profile::Change);
    std::hint::black_box((operation, change_is_stable));
}
