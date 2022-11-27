#![allow(non_snake_case)]

#[macro_use]
extern crate guard;

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
            i + 1,
            feature.properties.name,
            feature.properties.locality,
            feature.properties.county
        );
    }
}

fn print_departures(departures: &[EstimatedCall]) {
    for (_, call) in departures.iter().enumerate() {
        guard!(let Ok(expected_arrival) = DateTime::parse_from_rfc3339(&call.expectedArrivalTime) else {
            return;
        });

        let now: DateTime<chrono::Local> = chrono::offset::Local::now();
        let arrives_in_minutes = expected_arrival.signed_duration_since(now).num_minutes();

        match DateTime::parse_from_rfc3339(&call.expectedArrivalTime) {
            Ok(_time) => {}
            Err(_error) => {}
        }

        println!(
            "- {} \t:: {}",
            call.serviceJourney.journeyPattern.line.publicCode, call.destinationDisplay.frontText
        );

        if arrives_in_minutes > 10 {
            println!("  {}\n", expected_arrival.format("%H:%M"));
        } else {
            println!(
                "  {:?} min  [{}]\n",
                arrives_in_minutes,
                expected_arrival.format("%H:%M")
            );
        }
    }
}

fn get_user_input(input: &mut String) {
    let _ = stdout().flush();
    stdin()
        .read_line(input)
        .expect("Did not enter a correct string");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }
}

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();
    println!("Searching for {}", args.name);

    guard!(let Ok(geo_response) = btapi::request::geocoder::get_autocomplete_stop_name(&args.name).await else {
        return;
    });

    guard!(let Ok(geo) = serde_json::from_str::<btapi::model::Geocode>(&geo_response) else {
        return;
    });

    let mut input: String = String::new();

    if geo.features.len() > 1 {
        print_choices(&geo.features);

        println!("\nPick a number:");
        get_user_input(&mut input);
    } else {
        input = "1".to_string();
    }

    println!("\n----------------------------------\n[Departures]\n");

    guard!(let Ok(input) = input.parse::<usize>() else {
        println!("Valid inputs are: 1 - {}", geo.features.len());
        return;
    });

    let feature: &btapi::model::Feature = &geo.features[input - 1];

    guard!(let Ok(stopplace_response) = btapi::request::journeyplanner::stop_place(
        &feature.properties.id,
        &Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
    )
           .await else {
               return;
           });

    if let Ok(stopplace) = serde_json::from_str::<
        btapi::wrapper::Wrapper<btapi::model::StopPlaceResponse>,
    >(&stopplace_response)
    {
        print_departures(&stopplace.data.stopPlace.estimatedCalls);
    }
}
