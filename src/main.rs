#![allow(non_snake_case)]

#[macro_use]
extern crate guard;

use std::io::Write;

use btapi::model::EstimatedCall;
use chrono::{DateTime, Utc};
use clap::Parser;

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

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();
    println!("Searching for {}", args.name);

    let client = btapi::request::EnTurClient::new();

    guard!(let Ok(geo_response) = client.get_autocomplete_stop_name(&args.name).await else {
        println!("Could not find any stops using query: {}", args.name);
        return;
    });

    guard!(let Ok(geo) = serde_json::from_str::<btapi::model::Geocode>(&geo_response) else {
        // Could not parse response - panic/noop
        return;
    });

    let mut input: String = String::new();

    if geo.features.len() > 1 {
        print_choices(&geo.features);

        println!("\nPick a number:");
        btapi::helpers::get_user_input(&mut input);
    } else {
        input = "1".to_string();
    }

    let input = loop {
        match input.parse::<usize>() {
            Ok(input) => break input,
            Err(_error) => {
                print_choices(&geo.features);
                println!("\nInvalid number - pick another one");
                println!("Valid numbers are: 1 - {}\n", geo.features.len());

                let _ = std::io::stdout().flush();
                input = "".to_string();
                btapi::helpers::get_user_input(&mut input);
            }
        }
    };

    println!("\n----------------------------------\n[Departures]\n");

    let feature: &btapi::model::Feature = &geo.features[input - 1];

    guard!(let Ok(stopplace_response) = client.get_stop_place(
        &feature.properties.id,
        &Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
    ).await else {
        println!("Could not get any departures. Please try again later.");
        return;
    });

    if let Ok(stopplace) = serde_json::from_str::<
        btapi::wrapper::Wrapper<btapi::model::StopPlaceResponse>,
    >(&stopplace_response)
    {
        print_departures(&stopplace.data.stopPlace.estimatedCalls);
    }
}
