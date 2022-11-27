use http::header::{HeaderMap, HeaderValue};
use reqwest;

pub struct EnTurClient {
    http_client: reqwest::Client,
    base_url: String,
}

impl EnTurClient {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "ET-Client-Name",
            HeaderValue::from_static("tmnio-sanntidsappen-dev"),
        );

        let http_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let base_url = String::from("https://api.entur.io");

        Self {
            http_client,
            base_url,
        }
    }

    pub async fn get_stop_place(
        &self,
        stop_id: &str,
        start_time: &str,
    ) -> Result<String, reqwest::Error> {
        let url: String = format!("{}/journey-planner/v3/graphql", self.base_url);
        let query: String = format!(
            r#"
{{
    stopPlace(id: "{}") {{
        id
        name
        estimatedCalls(
            startTime: "{}",
            timeRange: 72100,
            numberOfDepartures: 50
        ) {{
            realtime
            aimedArrivalTime
            expectedArrivalTime
            date
            forBoarding
            destinationDisplay {{
                frontText
            }}
            quay {{
                id
                name
                publicCode
                description
            }}
            serviceJourney {{
                id
                journeyPattern {{
                    line {{
                        id
                        publicCode
                        name transportMode
                    }}
                }}
            }}
        }}
    }}
}}
"#,
            stop_id, start_time
        );

        let res: Result<reqwest::Response, reqwest::Error> =
            self.http_client.post(&url).body(query).send().await;

        let data: reqwest::Response = match res {
            Ok(response) => response,
            Err(error) => panic!("Request error: {}", error),
        };

        data.text().await
    }

    #[allow(dead_code)]
    pub async fn journey(
        &self,
        journey_id: &str,
        date_time: &str,
    ) -> Result<String, reqwest::Error> {
        let url: String = format!("{}//journey-planner/v3/graphql", self.base_url);
        let query: String = format!(
            r#"
{{
serviceJourney(id: "{}") {{
        estimatedCalls(date: "{}") {{
            aimedDepartureTime
            expectedDepartureTime
            quay {{
                id
                name
            }}
        }}
    }}
}}

"#,
            journey_id, date_time
        );

        let res = self.http_client.post(&url).body(query).send().await;

        let data: reqwest::Response = match res {
            Ok(response) => response,
            Err(error) => panic!("Request error: {}", error),
        };

        data.text().await
    }

    /// StopRegister API

    #[allow(dead_code)]
    pub async fn get_stop_info(&self, stop_id: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}/stop-places/v1/graphql", self.base_url);

        let query: String = format!(
            r#"
{{
    stopPlace(id: "{}", stopPlaceType: onstreetBus) {{
        id
        name {{
            value
        }}
        ... on StopPlace {{
            quays {{
                id
                compassBearing
                geometry {{
                    type
                    coordinates
                }}
            }}
        }}
    }}
}}
"#,
            stop_id
        );

        let res: Result<reqwest::Response, reqwest::Error> =
            self.http_client.post(&url).body(query).send().await;

        let data: reqwest::Response = match res {
            Ok(response) => response,
            Err(error) => panic!("Request error: {}", error),
        };

        data.text().await
    }

    /// Geocoder API
    pub async fn get_autocomplete_stop_name(&self, query: &str) -> Result<String, reqwest::Error> {
        let url = format!(
            "{}/geocoder/v1/autocomplete?text={}&layers=venue",
            self.base_url, query
        );

        let res = self.http_client.get(&url).send().await;

        let data = match res {
            Ok(response) => response,
            Err(error) => panic!("Request error: {}", error),
        };

        data.text().await
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub async fn get_nearby_stops(location: &haversinerust::Location) {}
}
