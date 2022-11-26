use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Quay {
    pub id: String,
    pub name: String,
    pub publicCode: String,
    pub description: Option<String>,
}
