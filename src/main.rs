#![allow(non_snake_case)]

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

fn print_departures(departures: &[EstimatedCall]) -> () {
    for (_, call) in departures.iter().enumerate() {
        let Ok(expected_departure) = DateTime::parse_from_rfc3339(&call.expectedDepartureTime) else {
            return;
        };

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

    let Ok(geo_response) = client.get_autocomplete_stop_name(&args.stop).await else {
        println!("Could not find any stops using query: {}", args.stop);
        return;
    };

    let Ok(geo) = serde_json::from_str::<btapi::model::Geocode>(&geo_response) else {
        // Could not parse response - panic/noop
        return;
    };

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

    let Ok(stopplace_response) = client.get_stop_place(&feature.properties.id, &now).await else {
        println!("Could not get any departures. Please try again later.");
        return;
    };

    // let test = serde_json::to_string(departures).unwrap();

    println!("{}", stopplace_response);

    if let Ok(stopplace) = serde_json::from_str::<
        btapi::wrapper::Wrapper<btapi::model::StopPlaceResponse>,
    >(&stopplace_response)
    {
        print_departures(&stopplace.data.stopPlace.estimatedCalls);
    }
}

async fn trip(client: &EnTurClient, args: &TripArgs) {
    let Ok(from_response) = client.get_autocomplete_stop_name(&args.from).await else {
        println!("Could not find any stops using query: {}", args.from);
        return;
    };

    let Ok(from) = serde_json::from_str::<btapi::model::Geocode>(&from_response) else {
        // Could not parse response - panic/noop
        return;
    };

    let Ok(to_response) = client.get_autocomplete_stop_name(&args.to).await else {
        println!("Could not find any stops using query: {}", args.to);
        return;
    };

    let Ok(to) = serde_json::from_str::<btapi::model::Geocode>(&to_response) else {
        // Could not parse response - panic/noop
        return;
    };

    let false = from.features.is_empty() else {
        println!("\x1b[31mX\x1b[0m Invalid stop: {} \x1b[1;36m", &args.to);
        return;
    };

    let false = to.features.is_empty() else {
        println!("\x1b[31mX\x1b[0m Invalid stop: {} \x1b[1;36m", &args.to);
        return;
    };

    let mut from_input: String = String::new();
    let mut to_input: String = String::new();

    if from.features.len() > 1 {
        println!("\x1b[1mTravel from\x1b[0m");
        print_choices(&from.features);

        println!();
        println!();
        print!(
            "\x1b[32m?\x1b[0m Which stop (1 - {}): \x1b[1;36m",
            from.features.len()
        );

        btapi::helpers::get_user_input(&mut from_input);
        print!("\x1b[0m");
    } else {
        from_input = "1".to_string();
    }

    let from_input = loop {
        match from_input.parse::<usize>() {
            Ok(value) => {
                if value > 0 && value <= from.features.len() {
                    break value;
                }
            }
            Err(_error) => { /* noop */ }
        }

        print!(
            "\x1b[31mX\x1b[0m Invalid stop - pick another one (1 - {}): \x1b[1;36m",
            from.features.len()
        );
        let _ = std::io::stdout().flush();
        from_input = "".to_string();
        btapi::helpers::get_user_input(&mut from_input);
        print!("\x1b[0m");
    };

    if to.features.len() > 1 {
        println!("\x1b[1mTravel to\x1b[0m");
        print_choices(&to.features);

        println!();
        println!();
        print!(
            "\x1b[32m?\x1b[0m Which stop (1 - {}): \x1b[1;36m",
            to.features.len()
        );

        btapi::helpers::get_user_input(&mut to_input);
        print!("\x1b[0m");
    } else {
        to_input = "1".to_string();
    }

    let to_input = loop {
        match to_input.parse::<usize>() {
            Ok(value) => {
                if value > 0 && value <= to.features.len() {
                    break value;
                }
            }
            Err(_error) => { /* noop */ }
        }

        print!(
            "\x1b[31mX\x1b[0m Invalid stop - pick another one (1 - {}): \x1b[1;36m",
            to.features.len()
        );
        let _ = std::io::stdout().flush();
        to_input = "".to_string();
        btapi::helpers::get_user_input(&mut to_input);
        print!("\x1b[0m");
    };

    let Ok(trip_response) = client.plan_trip(&from.features[from_input -1].properties.id, &to.features[to_input -1].properties.id).await else {
        println!("Error retrieving trip response");
        return;
    };

    println!();

    if let Ok(trip) =
        serde_json::from_str::<btapi::wrapper::Wrapper<btapi::model::TripResponse>>(&trip_response)
    {
        let patterns: Vec<btapi::model::TripPattern> = trip.data.trip.tripPatterns;

        for (_i, pattern) in patterns.iter().enumerate() {
            let duration = chrono::Duration::seconds(pattern.duration);
            let hours = duration.num_hours();
            let minutes = duration.num_minutes() - (hours * 60);

            print!("Travel time:");
            if hours > 0 {
                print!(" {} t", hours);
            }
            println!(" {} min", minutes);
            println!();

            for leg in &pattern.legs {
                // println!("Mode: {}", leg.mode);

                if let Some(from_estimated_call) = &leg.fromEstimatedCall {
                    if let Ok(expected_departure) =
                        DateTime::parse_from_rfc3339(&from_estimated_call.aimedDepartureTime)
                    {
                        print!("\x1b[1m{}\x1b[0m • ", expected_departure.format("%H:%M"));
                    }

                    print!("{}", from_estimated_call.quay.name);

                    if let Some(public_code) = &from_estimated_call.quay.publicCode {
                        print!(" \x1b[1mSpor {}\x1b[0m ", public_code);
                    }

                    println!();
                }

                if leg.mode == btapi::model::Mode::foot {
                    println!("      . ");
                    println!(
                        "      . Walk {} minutes",
                        chrono::Duration::seconds(leg.duration).num_minutes()
                    );
                    println!("      . ");
                } else {
                    println!("      |");
                    if let Some(line) = &leg.line {
                        print!("      | \x1b[97;42m {} \x1b[0m ", line.publicCode);
                    }

                    if let Some(to_estimated_call) = &leg.toEstimatedCall {
                        println!("{}", to_estimated_call.destinationDisplay.frontText);
                    }

                    println!(
                        "      | {} min",
                        chrono::Duration::seconds(leg.duration).num_minutes()
                    );
                    println!("      |");
                    println!("      |");
                }

                if let Some(to_estimated_call) = &leg.toEstimatedCall {
                    if let Ok(expected_departure) =
                        DateTime::parse_from_rfc3339(&to_estimated_call.aimedDepartureTime)
                    {
                        print!("\x1b[1m{}\x1b[0m • ", expected_departure.format("%H:%M"));
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
