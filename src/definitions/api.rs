#[derive(Debug, serde::Deserialize)]
pub struct APIResponseWrap<T> {
    pub code: u32,
    pub token: String,
    pub message: String,
    pub data: T,
}
