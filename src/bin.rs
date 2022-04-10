use anyhow::Error;
use clap::{Arg, Command};

use polygonify::{Framework, GeoType};

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
            return Err(anyhow::anyhow!(e));
        }
    };

    let mut frame = Framework::clap_constructor(matches);
    frame.describe();

    let poly = frame.build();

    match poly {
        Ok(GeoType::GeometryCollection(poly)) => {
            println!("This is a GeoCollection! {poly:#?}")
        }
        Ok(GeoType::Polygon(poly)) => println!("This is a Polygon! {poly:#?}"),
        Err(err) => return Err(err),
    };

    // let j = serde_json::to_string(&poly).expect("Bad JSON!");
    // println!("{j:#?}");

    return Ok(());
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
