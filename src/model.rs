#![allow(non_snake_case)]
#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DestinationDisplay {
    pub frontText: String,
}

#[derive(Deserialize, Debug)]
pub struct EstimatedCall {
    pub realtime: bool,
    pub aimedDepartureTime: String,
    pub expectedDepartureTime: String,
    pub date: String,
    pub forBoarding: bool,
    pub destinationDisplay: DestinationDisplay,
    pub quay: Quay,
    pub serviceJourney: Option<ServiceJourney>,
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
    pub publicCode: Option<String>,
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

/// Geocode types
#[derive(Deserialize, Debug)]
pub struct GeocodeResponse {
    pub geocoding: Geocode,
}

#[derive(Deserialize, Debug)]
pub struct Geocode {
    pub features: Vec<Feature>,
}

#[derive(Deserialize, Debug)]
pub struct Feature {
    pub geometry: Geometry,
    pub properties: Stop,
}

#[derive(Deserialize, Debug)]
pub struct Geometry {
    pub coordinates: [f32; 2],
}

#[derive(Deserialize, Debug)]
pub struct Stop {
    pub id: String,
    pub name: String,
    pub locality: String,
    pub county: String,
}

#[derive(Deserialize, Debug)]
pub struct TripResponse {
    pub trip: Trip,
}
#[derive(Deserialize, Debug)]
pub struct Trip {
    pub tripPatterns: Vec<TripPattern>,
}

#[derive(Deserialize, Debug)]
pub struct TripPattern {
    pub duration: i64,
    pub walkDistance: f64,
    pub legs: Vec<Leg>,
}

#[derive(Deserialize, Debug)]
pub struct Leg {
    pub expectedStartTime: String,
    pub expectedEndTime: String,
    pub duration: i64,
    pub mode: Mode,
    pub distance: f64,
    pub line: Option<Line>,
    pub fromEstimatedCall: Option<EstimatedCall>,
    pub toEstimatedCall: Option<EstimatedCall>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Mode {
    air,
    bicycle,
    bus,
    cableway,
    water,
    funicular,
    lift,
    rail,
    metro,
    tram,
    trolleybus,
    monorail,
    coach,
    foot,
    car,
    scooter,
}

use std::fmt;

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mode::air => write!(f, "Air"),
            Mode::bicycle => write!(f, "Bicycle"),
            Mode::bus => write!(f, "Bus"),
            Mode::cableway => write!(f, "Cableway"),
            Mode::water => write!(f, "Water"),
            Mode::funicular => write!(f, "Funicular"),
            Mode::lift => write!(f, "Lift"),
            Mode::rail => write!(f, "Rail"),
            Mode::metro => write!(f, "Metro"),
            Mode::tram => write!(f, "Tram"),
            Mode::trolleybus => write!(f, "Trolleybus"),
            Mode::monorail => write!(f, "Monorail"),
            Mode::coach => write!(f, "Coach"),
            Mode::foot => write!(f, "Foot"),
            Mode::car => write!(f, "Car"),
            Mode::scooter => write!(f, "Scooter"),
        }
    }
}
