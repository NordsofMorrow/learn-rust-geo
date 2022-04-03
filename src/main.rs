extern crate geo;

mod lib;
use polygonify::{Framework, GeoType};

extern crate clap;
use clap::{Arg, Command, Error};

use geo::GeoNum;
use num_traits::{Num, NumCast};

extern crate geojson;
extern crate rand;
extern crate serde;
extern crate serde_json;

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
                .validator(|s| s.parse::<usize>())
                .help("Maximum polygon vertices"),
        )
        .arg(
            Arg::new("convex_hull")
                .short('h')
                .long("hull")
                .takes_value(false)
                .help("Use the convex hull of the polygon"),
        )
        .arg(
            Arg::new("collection")
                .short('c')
                .long("collection")
                .takes_value(false)
                .help("Return a GeometryCollection"),
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

    let ends = constructor::<f64>(matches);

    // let j = serde_json::to_string(&poly).expect("Bad JSON!");
    // println!("{j:#?}");

    return Ok(ends);
}

fn constructor<T>(matches: clap::ArgMatches) -> ()
where
    T: std::ops::Mul<Output = T>
        + Clone
        + Num
        + NumCast
        + Copy
        + PartialOrd
        + GeoNum
        + std::fmt::Display
        + std::fmt::Debug
        + std::str::FromStr
        + rand::distributions::uniform::SampleUniform,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let lons: Vec<T> = matches.values_of_t("x").expect("Needs lon boundaries!");
    let lats: Vec<T> = matches.values_of_t("y").expect("Needs lat boundaries!");
    let vertices: usize = matches.value_of_t("z").expect("Need vertices!");
    let convex_hull = matches.is_present("convex_hull");
    let gc = matches.is_present("collection");

    let f: Framework<T> = Framework::new(lons, lats, vertices);
    f.describe();

    let poly = f.build(gc, convex_hull);
    match poly {
        Some(GeoType::GeometryCollection(..)) => println!("This is GeoCollection! {poly:#?}"),
        Some(GeoType::Polygon(..)) => println!("This is Polygon! {poly:#?}"),
        None => panic!("This should never happen"),
    };
    ()
}

// fn make_poly() {
//     let p = Framework {
//         lon_min: 5.5,
//         lon_max: 6.5,
//         lat_min: 1.2,
//         lat_max: 1000.,
//         vertices: 10,
//     };

//     let poly: Polygon<f64> = p.build_polygon(true);
//     println!("{poly:#?}")
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
