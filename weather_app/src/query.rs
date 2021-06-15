pub struct Query{
    pub city: String,
    pub language: Language,
    pub units: Units,
}

impl Query{
    pub fn new(city: &str, language: Language, units: Units) -> Query {
        Query{
            city: String::from(city),
            language,
            units,
        }
    }
}

pub enum Language{
    English,
    Polish,
}

impl Language {
    pub fn to_string(&self) -> String {
        match self{
            Language::English => String::from("en"),
            Language::Polish => String::from("pl"),
        }
    }
}

pub enum Units{
    Metric,
    Imperial,
}

impl Units {
    pub fn to_string(&self) -> String {
        match self{
            Units::Metric => String::from("metric"),
            Units::Imperial => String::from("imperial"),
        }
    }
}