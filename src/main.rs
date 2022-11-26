#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

use chrono;
use clap::Parser;
use std::io::{stdin, stdout, Write};

mod btapi;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
    // #[arg(short, long)]
    // count: u8,
}

#[tokio::main]
async fn main() -> serde_json::Result<()> {
    let args = Args::parse();
    println!("Autocompleting {} \n", args.name);

    let geo_response = btapi::request::geocoder::get_autocomplete_stop_name(&args.name)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let geo: btapi::model::Geocode = serde_json::from_str(&geo_response).unwrap();

    for (i, feature) in geo.features.iter().enumerate() {
        println!(
            "{} - {} [{} - {}]",
            i, feature.properties.name, feature.properties.locality, feature.properties.county
        );
    }

    let mut input = String::new();

    println!("\nWhich one did you mean?");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    // println!("You picked: {} - {}", input, chrono::offset::Local::now());
    println!("");

    let inputNum: usize = input.parse::<usize>().unwrap();
    let feature: &btapi::model::Feature = &geo.features[inputNum];

    let res = btapi::request::journeyplanner::stop_place(
        &feature.properties.id,
        &chrono::offset::Local::now()
            .format("%Y-%m-%dT%H:%M:%SZ")
            .to_string(),
    )
    .await
    .unwrap()
    .text()
    .await
    .unwrap();
    // println!("{}", res);

    let wrapper: btapi::wrapper::Wrapper<btapi::model::StopPlaceResponse> =
        serde_json::from_str(&res)?;

    for (i, call) in wrapper.data.stopPlace.estimatedCalls.iter().enumerate() {
        println!(
            "{} {}",
            call.serviceJourney.journeyPattern.line.publicCode, call.destinationDisplay.frontText
        );
        println!("{}\n", call.aimedArrivalTime);
    }
    //     println!("{:?}", wrapper.data);

    Ok(())
}
