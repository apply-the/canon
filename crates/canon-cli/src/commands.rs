pub mod approve;
pub mod init;
pub mod inspect;
pub mod resume;
pub mod run;
pub mod skills;
pub mod status;
pub mod verify;

pub fn exit_code_for_state(state: &str) -> i32 {
    match state {
        "Blocked" => 2,
        "AwaitingApproval" => 3,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::exit_code_for_state;

    #[test]
    fn exit_code_maps_blocked_and_approval_states() {
        assert_eq!(exit_code_for_state("Blocked"), 2);
        assert_eq!(exit_code_for_state("AwaitingApproval"), 3);
        assert_eq!(exit_code_for_state("Completed"), 0);
    }
}
