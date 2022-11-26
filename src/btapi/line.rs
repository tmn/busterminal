use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Line {
    pub id: String,
    pub publicCode: String,
    pub name: String,
    pub transportMode: String,
}
