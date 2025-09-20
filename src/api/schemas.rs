use serde::{Deserialize, Serialize};

pub trait HashGenerate {
    fn get_hash(&self) -> String;
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct PublicInfoRequest {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
}

impl HashGenerate for PublicInfoRequest {
    fn get_hash(&self) -> String {
        format!(
            "{}{}{}",
            self.first_name.to_ascii_uppercase(),
            self.last_name.to_ascii_uppercase(),
            self.middle_name
                .clone()
                .unwrap_or_default()
                .to_ascii_uppercase()
        )
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct PassportInfoRequest {
    pub passport_number: i32,
    pub passport_series: i32,
}

impl HashGenerate for PassportInfoRequest {
    fn get_hash(&self) -> String {
        format!("{}{}", self.passport_number, self.passport_series)
    }
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct MockResponseBody {
    pub service_id: i32,
    pub data_hash: String,
    pub data: String,
}

#[derive(Debug, Deserialize)]
pub struct MockResponseRequest {
    pub data: String,
}
