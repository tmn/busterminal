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
    stop: String,
}

fn print_choices(features: &[btapi::model::Feature]) {
    for (i, feature) in features.iter().enumerate() {
        println!(
            "\x1b[32m{}\x1b[0m - \x1b[1m{}\x1b[0m ({} - {})",
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
        let arrives_in_minutes: i64 = expected_arrival.signed_duration_since(now).num_minutes();
        let expected_arrival_formated = expected_arrival.format("%H:%M");

        println!(
            " \x1b[97;42;1m {} \x1b[0m {}",
            call.serviceJourney.journeyPattern.line.publicCode, call.destinationDisplay.frontText
        );

        if arrives_in_minutes > 10 {
            println!(" \x1b[1m{}\x1b[0m", expected_arrival_formated);
        } else {
            println!(
                " \x1b[1m{:?} min\x1b[0m ({})",
                arrives_in_minutes, expected_arrival_formated
            );
        }

        println!();
    }
}

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();
    let client: btapi::request::EnTurClient = btapi::request::EnTurClient::new();

    println!("Searching for \x1b[32;1m{}\x1b[0m", args.stop);
    println!();

    guard!(let Ok(geo_response) = client.get_autocomplete_stop_name(&args.stop).await else {
        println!("Could not find any stops using query: {}", args.stop);
        return;
    });

    guard!(let Ok(geo) = serde_json::from_str::<btapi::model::Geocode>(&geo_response) else {
        // Could not parse response - panic/noop
        return;
    });

    let mut input: String = String::new();

    if geo.features.len() > 1 {
        print_choices(&geo.features);

        println!();
        println!();
        print!("\x1b[32m?\x1b[0m Which stop (1 - {}): ", geo.features.len());

        btapi::helpers::get_user_input(&mut input);
    } else {
        input = "1".to_string();
    }

    let input = loop {
        match input.parse::<usize>() {
            Ok(value) => {
                if value > 0 && value <= geo.features.len() {
                    break value;
                }
            }
            Err(_error) => { /* noop */ }
        }

        print!(
            "\x1b[31mX\x1b[0m Invalid stop - pick another one (1 - {}): ",
            geo.features.len()
        );
        let _ = std::io::stdout().flush();
        input = "".to_string();
        btapi::helpers::get_user_input(&mut input);
    };

    println!();
    println!("----------------------------------");
    println!();
    println!(
        "\x1b[1mDepartures for \x1b[4m{} ({} - {})\x1b[0m",
        geo.features[input - 1].properties.name,
        geo.features[input - 1].properties.locality,
        geo.features[input - 1].properties.county,
    );
    println!();

    let feature: &btapi::model::Feature = &geo.features[input - 1];
    let now: String = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

    guard!(let Ok(stopplace_response) = client.get_stop_place(&feature.properties.id, &now).await else {
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
