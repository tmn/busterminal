use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DestinationDisplay {
    pub frontText: String,
}

#[derive(Deserialize, Debug)]
pub struct EstimatedCall {
    pub realtime: bool,
    pub aimedArrivalTime: String,
    pub expectedArrivalTime: String,
    pub date: String,
    pub forBoarding: bool,
    pub destinationDisplay: DestinationDisplay,
    pub quay: Quay,
    pub serviceJourney: ServiceJourney,
}

#[derive(Deserialize, Debug)]
pub struct JourneyPattern {
    pub line: Line,
}

#[derive(Deserialize, Debug)]
pub struct Line {
    pub id: String,
    pub publicCode: String,
    pub name: String,
    pub transportMode: String,
}

#[derive(Deserialize, Debug)]
pub struct Quay {
    pub id: String,
    pub name: String,
    pub publicCode: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ServiceJourney {
    pub id: String,
    pub journeyPattern: JourneyPattern,
}

#[derive(Deserialize, Debug)]
pub struct StopPlace {
    pub id: String,
    pub name: String,
    pub estimatedCalls: Vec<EstimatedCall>,
}

#[derive(Deserialize, Debug)]
pub struct StopPlaceResponse {
    pub stopPlace: StopPlace,
}
