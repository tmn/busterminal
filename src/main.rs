mod btapi;

#[tokio::main]
async fn main() -> serde_json::Result<()> {
    let res = btapi::btapi::journey_planner("NSR:StopPlace:62392", "2022-11-22T22:47:11Z")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{}", res);

    let wrapper: btapi::wrapper::Wrapper<btapi::stopplaceresponse::StopPlaceResponse> =
        serde_json::from_str(&res)?;
    println!("{:?}", wrapper.data);

    Ok(())
}
