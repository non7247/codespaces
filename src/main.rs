use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, USER_AGENT};
use serde::Deserialize;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Deserialize)]
struct WikiDataAttraction {
    value: String,
}

impl WikiDataAttraction {
    fn get_local_name(&self) -> String {
        let idx = self.value.rfind("/");
        match idx {
            Some(idx) => { self.value[idx + 1..].to_string() },
            None => String::new()
        }
    }
}

#[derive(Debug, Deserialize)]
struct WikidataLabel {
    value: String,
}

#[derive(Debug, Deserialize)]
struct Binding {
    attraction: WikiDataAttraction,
    label: WikidataLabel,
}

#[derive(Debug, Deserialize)]
struct Results {
    bindings: Vec<Binding>,
}

#[derive(Debug, Deserialize)]
struct Solution {
    results: Results,
}

fn main() -> Result<()> {
    let client = Client::new();
    let url = "https://query.wikidata.org/sparql";
    let user_agent = "User-Agent: Other";

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, user_agent.parse().unwrap());

    let response = client
        .get(url)
        .headers(headers)
        .query(&[
            ("format", "json"),
            ("query", format!("
PREFIX wd: <http://www.wikidata.org/entity/>
PREFIX wdt: <http://www.wikidata.org/prop/direct/>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
SELECT DISTINCT ?attraction ?label WHERE {{
    ?attraction wdt:P31 wd:Q570116;
                rdfs:label ?label.
    FILTER(LANG(?label) = \"en\").
}} LIMIT {}
            ", 3).trim())
        ])
        .send()?;

    let body = response.text()?;
    println!("{}", body);

    let parsed_json: Solution = serde_json::from_str(&body)?;
    println!("{:?}", parsed_json);

    for binding in parsed_json.results.bindings.iter() {
        println!("\"{}\", \"{}\"", binding.attraction.get_local_name(), binding.label.value);
    }

    Ok(())
}