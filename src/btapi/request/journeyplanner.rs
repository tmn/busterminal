use reqwest;

pub async fn stop_place(
    stop_id: &str,
    start_time: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();

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
    )
    .to_string();

    let res = client
        .post("https://api.entur.io/journey-planner/v3/graphql")
        .header("ET-Client-Name", "tmnio-sanntidsappen-dev")
        .body(query)
        .send()
        .await?;

    Ok(res)
}

pub async fn journey(
    journey_id: &str,
    date_time: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();

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
    )
    .to_string();

    let res = client
        .post("https://api.entur.io/journey-planner/v3/graphql")
        .header("ET-Client-Name", "tmnio-sanntidsappen-dev")
        .body(query)
        .send()
        .await?;

    Ok(res)
}
