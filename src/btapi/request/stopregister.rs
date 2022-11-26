use reqwest;

pub async fn journey(stop_id: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();

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
    )
    .to_string();

    let res = client
        .post("https://api.entur.io/stop-places/v1/graphql")
        .header("ET-Client-Name", "tmnio-sanntidsappen-dev")
        .body(query)
        .send()
        .await?;

    Ok(res)
}
