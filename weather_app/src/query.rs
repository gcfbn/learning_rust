use std::fmt;

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

impl fmt::Display for Language{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Language::English => write!(f, "en"),
            Language::Polish => write!(f, "pl"),
        }
    }
}

pub enum Units{
    Metric,
    Imperial,
}

impl fmt::Display for Units{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Units::Metric => write!(f, "metrics"),
            Units::Imperial => write!(f, "imperial"),
        }
    }
}