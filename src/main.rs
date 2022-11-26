mod btapi;

#[tokio::main]
async fn main() -> serde_json::Result<()> {
    let res =
        btapi::request::journeyplanner::stop_place("NSR:StopPlace:62392", "2022-11-22T22:47:11Z")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
    println!("{}", res);

    let wrapper: btapi::wrapper::Wrapper<btapi::model::StopPlaceResponse> =
        serde_json::from_str(&res)?;
    println!("{:?}", wrapper.data);

    let geocoder_query = "sandvika";
    let res2 = btapi::request::geocoder::get_autocomplete_stop_name(&geocoder_query)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{}", res2);

    Ok(())
}
