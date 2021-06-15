use clap::clap_app;

fn main() {
    let matches = clap_app!(weather_app =>
        (version: "1.0")
        (author: "Bartek M. <bmekarski@interia.pl>")
        (about: "Weather app using free API")
        (@arg POLISH: -p --polish "Use polish instead of default english language")
        (@arg IMPERIAL: -i --imperial "Use imperial units instead of default metrics units")
        (@arg CITY: -c --city +required +takes_value "Search for weather in city")
    )
        .get_matches();

    println!("Hello, world!");
}
