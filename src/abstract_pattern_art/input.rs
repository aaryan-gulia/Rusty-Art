use clap::{Command, Arg};
use bevy::prelude::Resource;
use clap::ArgMatches;

pub struct ArgMatchesResource(pub ArgMatches);
impl Resource for ArgMatchesResource {}

pub fn get_input() -> ArgMatchesResource{
    let app = Command::new("Generative Art")
        .version("1.0")
        .author("Your Name")
        .arg(Arg::new("shape")
            .short('s')
            .long("shape")
            .value_name("SHAPE")
            .help("Select the shape (circle, square, triangle)"))
        .arg(Arg::new("size_min")
            .short('m')
            .long("size_min")
            .value_name("SIZE")
            .help("Minimum size of the shapes"))
        .arg(Arg::new("size_max")
            .short('M')
            .long("size_max")
            .value_name("SIZE")
            .help("Maximum size of the shapes"))
        // Add arguments for density, randomness, complexity etc.
        .get_matches();

    ArgMatchesResource(app)
}