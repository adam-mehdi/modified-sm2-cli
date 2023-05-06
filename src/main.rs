use clap::{App, Arg};
use std::process;

mod sm;

fn main() {
    let matches = App::new("Spaced Repetition Scheduler")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("A CLI to calculate spaced repetition intervals")
        .arg(
            Arg::with_name("qualities")
                .short('q')
                .long("qualities")
                .value_name("QUALITIES")
                .help("Quality of subsequent responses (1-5)")
                .required(true)
                .multiple(true)
                .takes_value(true),
        )
        .get_matches();



    let qualities: Vec<i32> = matches
    .values_of("qualities")
    .unwrap()
    .map(|q| {
        q.parse().unwrap_or_else(|err| {
            eprintln!("Error: {}", err);
            process::exit(1);
        })
    })
    .collect();
    
    let mut current_reps = 0;
    let mut current_easiness = 2.5;
    let mut current_interval = 0;
    
    for (i, quality) in qualities.iter().enumerate() {
        let sm_response = sm::calculate_sm(*quality, current_reps, current_interval, current_easiness);
        println!("Interval after response {}: {}", i + 1, sm_response.interval);
    
        current_reps = sm_response.repetitions;
        current_easiness = sm_response.ease_factor;
        current_interval = sm_response.interval;
    }

}
