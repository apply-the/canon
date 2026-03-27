pub mod approve;
pub mod init;
pub mod inspect;
pub mod resume;
pub mod run;
pub mod status;
pub mod verify;

pub fn exit_code_for_state(state: &str) -> i32 {
    match state {
        "Blocked" => 2,
        "AwaitingApproval" => 3,
        _ => 0,
    }
}
