#![allow(non_snake_case)]

#[macro_use]
extern crate guard;

use std::io::Write;

use btapi::{model::EstimatedCall, request::EnTurClient};
use chrono::{DateTime, Utc};
use clap::{Args, Parser};

mod btapi;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Args, Debug)]
struct DepartureArgs {
    #[arg(short, long)]
    stop: String,
}

#[derive(Args, Debug)]
struct TripArgs {
    #[arg(short, long)]
    from: String,

    #[arg(short, long)]
    to: String,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    Departure(DepartureArgs),
    Trip(TripArgs),
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
        guard!(let Ok(expected_departure) = DateTime::parse_from_rfc3339(&call.expectedDepartureTime) else {
            return;
        });

        let now: DateTime<chrono::Local> = chrono::offset::Local::now();
        let arrives_in_minutes: i64 = expected_departure.signed_duration_since(now).num_minutes();
        let expected_departure_formated = expected_departure.format("%H:%M");

        if let Some(service_journey) = &call.serviceJourney {
            print!(
                "\x1b[97;42;1m {} \x1b[0m",
                service_journey.journeyPattern.line.publicCode
            );
        }

        println!(" {}", call.destinationDisplay.frontText);

        if arrives_in_minutes > 10 {
            println!(" \x1b[1m{}\x1b[0m", expected_departure_formated);
        } else {
            println!(
                " \x1b[1m{:?} min\x1b[0m ({})",
                arrives_in_minutes, expected_departure_formated
            );
        }

        println!();
    }
}

async fn departure(client: &EnTurClient, args: &DepartureArgs) {
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
        print!(
            "\x1b[32m?\x1b[0m Which stop (1 - {}): \x1b[1;36m",
            geo.features.len()
        );

        btapi::helpers::get_user_input(&mut input);
        print!("\x1b[0m");
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
            "\x1b[31mX\x1b[0m Invalid stop - pick another one (1 - {}): \x1b[1;36m",
            geo.features.len()
        );
        let _ = std::io::stdout().flush();
        input = "".to_string();
        btapi::helpers::get_user_input(&mut input);
        print!("\x1b[0m");
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

async fn trip(client: &EnTurClient, args: &TripArgs) {
    guard!(let Ok(trip_response) = client.plan_trip(&args.from, &args.to).await else {
        println!("Error retrieving trip response");
        return;
    });

    if let Ok(trip) =
        serde_json::from_str::<btapi::wrapper::Wrapper<btapi::model::TripResponse>>(&trip_response)
    {
        let patterns: Vec<btapi::model::TripPattern> = trip.data.trip.tripPatterns;

        for (_i, pattern) in patterns.iter().enumerate() {
            let duration = chrono::Duration::seconds(pattern.duration);
            let hours = duration.num_hours();
            let minutes = duration.num_minutes() - (hours * 60);

            print!("Duration:");
            if hours > 0 {
                print!(" {} t", hours);
            }
            println!(" {} min", minutes);
            println!();

            for leg in &pattern.legs {
                println!("Mode: {}", leg.mode);

                if let Some(from_estimated_call) = &leg.fromEstimatedCall {
                    if let Ok(expected_departure) =
                        DateTime::parse_from_rfc3339(&from_estimated_call.aimedDepartureTime)
                    {
                        print!("\x1b[1m{}\x1b[0m ", expected_departure.format("%H:%M"));
                    }

                    print!("{}", from_estimated_call.quay.name);

                    if let Some(public_code) = &from_estimated_call.quay.publicCode {
                        print!(" \x1b[1mSpor {}\x1b[0m ", public_code);
                    }

                    println!();
                }

                println!("|");
                println!(
                    "{} min",
                    chrono::Duration::seconds(leg.duration).num_minutes()
                );
                println!("|");

                if let Some(to_estimated_call) = &leg.toEstimatedCall {
                    if let Ok(expected_departure) =
                        DateTime::parse_from_rfc3339(&to_estimated_call.aimedDepartureTime)
                    {
                        print!("\x1b[1m{}\x1b[0m ", expected_departure.format("%H:%M"));
                    }

                    print!("{} ", to_estimated_call.quay.name);

                    if let Some(public_code) = &to_estimated_call.quay.publicCode {
                        print!(" \x1b[1mSpor {}\x1b[0m ", public_code);
                    }

                    println!();
                }
                println!();
            }

            println!(
                "================================================================================"
            );
            println!();
        }
    }
}

#[tokio::main]
async fn main() {
    let cli: Cli = Cli::parse();
    let client: btapi::request::EnTurClient = btapi::request::EnTurClient::new();

    match &cli.action {
        Action::Departure(args) => departure(&client, args).await,
        Action::Trip(args) => trip(&client, args).await,
    };
}
