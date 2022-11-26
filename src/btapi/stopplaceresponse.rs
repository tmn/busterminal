use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StopPlaceResponse {
    pub stopPlace: super::stopplace::StopPlace,
}
