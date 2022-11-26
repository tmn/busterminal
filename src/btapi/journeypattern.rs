use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct JourneyPattern {
    pub line: super::line::Line,
}
