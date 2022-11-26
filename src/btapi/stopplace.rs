use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StopPlace {
    pub id: String,
    pub name: String,
    pub estimatedCalls: Vec<super::estimatedcall::EstimatedCall>,
}
