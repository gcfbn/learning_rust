use clap::clap_app;

use weather_app::query;

fn main() {
    let matches = clap_app!(weather_app =>
        (version: "1.0")
        (author: "Bartek M. <bmekarski@interia.pl>")
        (about: "Weather app using free API")
        (@arg polish: -p --polish "Use polish instead of default english language")
        (@arg imperial: -i --imperial "Use imperial units instead of default metrics units")
        (@arg CITY: -c --city +required +takes_value "Search for weather in city")
    )
        .get_matches();

    let language = match matches.occurrences_of("polish") {
        0 => query::Language::English,
        _ => query::Language::Polish,
    };

    let units = match matches.occurrences_of("imperial") {
        0 => query::Units::Metric,
        _ => query::Units::Imperial,
    };

    // unwrap is safe because "CITY" is required
    let city = matches.value_of("CITY").unwrap();

    let query = query::Query::new(city, language, units);

    let result = weather_app::run(&query);

    match result {
        Ok(data) => println!("{:#?}", data),
        Err(error) => println!("An error has occurred: \n{:#?}", error),
    }
}
