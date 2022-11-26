use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EstimatedCall {
    pub realtime: bool,
    pub aimedArrivalTime: String,
    pub expectedArrivalTime: String,
    pub date: String,
    pub forBoarding: bool,
    pub destinationDisplay: super::destinationdisplay::DestinationDisplay,
    pub quay: super::qyay::Quay,
    pub serviceJourney: super::servicejourney::ServiceJourney,
}
