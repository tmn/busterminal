use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ServiceJourney {
    pub id: String,
    pub journeyPattern: super::journeypattern::JourneyPattern,
}
