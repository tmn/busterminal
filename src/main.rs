#![allow(non_snake_case)]

use btapi::model::EstimatedCall;
use chrono::{DateTime, Utc};
use clap::Parser;
use std::io::{stdin, stdout, Write};

mod btapi;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
}

fn print_choices(features: &[btapi::model::Feature]) {
    for (i, feature) in features.iter().enumerate() {
        println!(
            "{} - {} [{} - {}]",
            i, feature.properties.name, feature.properties.locality, feature.properties.county
        );
    }
}

fn print_departures(departures: &[EstimatedCall]) {
    for (_, call) in departures.iter().enumerate() {
        let now = chrono::offset::Local::now();
        let expected_arrival = DateTime::parse_from_rfc3339(&call.expectedArrivalTime).unwrap();
        let arrives_in_minutes = expected_arrival.signed_duration_since(now).num_minutes();

        println!(
            "{} {}",
            call.serviceJourney.journeyPattern.line.publicCode, call.destinationDisplay.frontText
        );

        if arrives_in_minutes > 10 {
            println!("{}\n", expected_arrival.format("%H:%M"));
        } else {
            println!("{:?} min\n", arrives_in_minutes);
        }
    }
}

#[tokio::main]
async fn main() -> serde_json::Result<()> {
    let args = Args::parse();
    println!("Searching for {}", args.name);

    let geo_response = btapi::request::geocoder::get_autocomplete_stop_name(&args.name)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let geo: btapi::model::Geocode = serde_json::from_str(&geo_response).unwrap();
    let mut input = String::new();

    if geo.features.len() > 1 {
        print_choices(&geo.features);

        println!("\nPick a number:");

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
    } else {
        input = "0".to_string();
    }

    println!("\n----------------------------------\n[Departures]\n");

    let feature: &btapi::model::Feature = &geo.features[input.parse::<usize>().unwrap()];
    let res = btapi::request::journeyplanner::stop_place(
        &feature.properties.id,
        &Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
    )
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

    let stopplace_response: btapi::wrapper::Wrapper<btapi::model::StopPlaceResponse> =
        serde_json::from_str(&res)?;
    print_departures(&stopplace_response.data.stopPlace.estimatedCalls);

    Ok(())
}
