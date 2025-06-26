use crate::definitions::models::Account;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    #[serde(rename = "changementMDP")]
    pub password_change: bool,
    #[serde(rename = "nbJourMdpExire")]
    pub days_until_password_expires: u32,
    pub accounts: Vec<Account>,
}

#[cfg(test)]
mod tests {
    use super::LoginResponse;
    use crate::definitions::api::APIResponseWrap;
    use std::fs;

    #[test]
    fn can_read_student_account() {
        let data =
            fs::read_to_string("tests/student_login.json").expect("failed to read json file");

        let parsed: APIResponseWrap<LoginResponse> =
            serde_json::from_str(&data).expect("failed to parse json");

        println!("{:#?}", parsed);
    }

    #[test]
    fn can_read_student_primary_account() {
        let data =
            fs::read_to_string("tests/studentp_login.json").expect("failed to read json file");

        let parsed: APIResponseWrap<LoginResponse> =
            serde_json::from_str(&data).expect("failed to parse json");

        println!("{:#?}", parsed);
    }

    #[test]
    fn can_read_family_account() {
        let data = fs::read_to_string("tests/family_login.json").expect("failed to read json file");

        let parsed: APIResponseWrap<LoginResponse> =
            serde_json::from_str(&data).expect("failed to parse json");

        println!("{:#?}", parsed);
    }
}
