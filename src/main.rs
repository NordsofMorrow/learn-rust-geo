extern crate geo;

mod lib;
use polygonify::{Framework, Polygon};

extern crate clap;
use clap::{Arg, Command, Error};

fn main() -> Result<(), Error> {
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");

    let app = Command::new("Polygonify")
        .version(version)
        .author(authors)
        .about("Generate bounded polygons using Rust!")
        .arg_required_else_help(true)
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .max_occurrences(2)
                .multiple_occurrences(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::new("version")
                .long("version")
                .help("Prints the version of Polygonify"),
        )
        .arg(
            Arg::new("x")
                .short('x')
                .long("lon_bounds")
                .multiple_values(true)
                .number_of_values(2)
                .required(true)
                .use_value_delimiter(true)
                .validator(|s| s.parse::<f64>())
                .help("Longitude boundaries of the polygon"),
        )
        .arg(
            Arg::new("y")
                .short('y')
                .long("lat_bounds")
                .multiple_values(true)
                .number_of_values(2)
                .required(true)
                .use_value_delimiter(true)
                .validator(|s| s.parse::<f64>())
                .help("Latitude boundaries of the polygon"),
        )
        .arg(
            Arg::new("z")
                .short('z')
                .long("vertices")
                .takes_value(true)
                .multiple_occurrences(true)
                .required(false)
                .default_value("3")
                .validator(|s| s.parse::<i32>())
                .help("Maximum polygon vertices"),
        )
        .arg(
            Arg::new("convex_hull")
                .short('h')
                .long("hull")
                .takes_value(false)
                .help("Use the convex hull of the polygon"),
        );

    let matches = match app.try_get_matches() {
        Ok(matches) => matches,
        Err(e) if e.kind() == clap::ErrorKind::MissingRequiredArgument => {
            println!("Polygonify failed to run: {:?}!", e.kind());
            return Ok(());
        }
        Err(e)
            if e.kind() == clap::ErrorKind::DisplayHelp
                || e.kind() == clap::ErrorKind::DisplayVersion
                || e.kind() == clap::ErrorKind::ValueValidation
                || e.kind() == clap::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand =>
        {
            e.exit()
        }

        Err(e) => {
            println!("Something went really wrong!");
            return Err(e);
        }
    };

    let lons: Vec<f64> = matches.values_of_t("x").expect("Needs lon boundaries!");
    for (i, l) in lons.into_iter().enumerate() {
        println!("{}, {}", &i, &l)
    }
    let lats: Vec<f64> = matches.values_of_t("y").expect("Needs lat boundaries!");
    for (i, l) in lats.into_iter().enumerate() {
        println!("{}, {}", &i, &l)
    }

    let vertices: i32 = matches.value_of_t("z").expect("Need vertices!");
    println!("Value for vertices: {}", vertices);

    return Ok(());
}

fn make_poly() {
    let p = Framework {
        lon_min: 5.5,
        lon_max: 6.5,
        lat_min: 1.2,
        lat_max: 1000.,
        vertices: 10,
    };

    let poly: Polygon<f64> = p.build_polygon(true);
    println!("{:#?}", poly)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
