use haversinerust;
use reqwest;

pub async fn get_autocomplete_stop_name(query: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();

    let res = client
        .get(format!(
            "https://api.entur.io/geocoder/v1/autocomplete?text={}&layers=venue",
            query
        ))
        .header("ET-Client-Name", "tmnio-sanntidsappen-dev")
        .send()
        .await?;

    Ok(res)
}

pub async fn get_nearby_stops(location: &haversinerust::Location) {}
