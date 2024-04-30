use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, USER_AGENT};
use serde::Deserialize;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Deserialize)]
struct WDInstance {
    value: String,
}

impl WDInstance {
    fn get_local_name(&self) -> String {
        let idx = self.value.rfind("/");
        match idx {
            Some(idx) => { self.value[idx + 1..].to_string() },
            None => String::new()
        }
    }
}

#[derive(Debug, Deserialize)]
struct WDLabel {
    value: String,
}

#[derive(Debug, Deserialize)]
struct WDNumber {
    value: String,
}

impl WDNumber {
    fn get_number(&self) -> u64 {
        match self.value.parse::<u64>() {
            Ok(n) => n,
            Err(_) => 0,
        }
    }
}

#[derive(Debug, Deserialize)]
struct BindingAttraction {
    attraction: WDInstance,
    #[serde(rename = "attractionLabel")]
    attraction_label: WDLabel,
    description: Option<String>,
    location: WDInstance,
    #[serde(rename = "locationLabel")]
    location_label: WDLabel,
    population: WDNumber,
}

#[derive(Debug, Deserialize)]
struct ResultsAttraction {
    bindings: Vec<BindingAttraction>,
}

#[derive(Debug, Deserialize)]
struct SolutionAttraction {
    results: ResultsAttraction,
}

#[derive(Debug, Deserialize)]
struct BindingArtist {
    artist: WDInstance,
    #[serde(rename = "artistLabel")]
    artist_label: WDLabel,
    followers: WDNumber,
}

#[derive(Debug, Deserialize)]
struct ResultsArtist {
    bindings: Vec<BindingArtist>,
}

#[derive(Debug, Deserialize)]
struct SolutionArtist {
    results: ResultsArtist,
}

#[derive(Debug, Deserialize)]
struct BindingSubject {
    subject: WDInstance,
    #[serde(rename = "subjectLabel")]
    subject_label: WDLabel,
    #[serde(rename = "boxOffice")]
    box_office: WDNumber,
}

#[derive(Debug, Deserialize)]
struct ResultSubject {
    bindings: Vec<BindingSubject>,
}

#[derive(Debug, Deserialize)]
struct SolutionSubject {
    results: ResultSubject,
}

fn find_attraction(client: &Client, url: &str, headers: HeaderMap) -> Result<Vec<BindingAttraction>> {
    let response = client
        .get(url)
        .headers(headers)
        .query(&[
            ("format", "json"),
            ("query", format!("
PREFIX wd: <http://www.wikidata.org/entity/>
PREFIX wdt: <http://www.wikidata.org/prop/direct/>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
PREFIX schema: <http://schema.org>

SELECT DISTINCT ?attraction ?attractionLabel ?description
                ?location ?locationLabel ?population WHERE {{
    ?attraction wdt:P31 wd:Q570116;
                rdfs:label ?attractionLabel;
                wdt:P131 ?location.
    FILTER(LANG(?attractionLabel) = \"en\").

    OPTIONAL{{
        ?attraction schema:description ?description.
        FILTER(LANG(?description) = \"en\").
    }}
                    
    ?location wdt:P1082 ?population;
            rdfs:label ?locationLabel;
    FILTER(LANG(?locationLabel) = \"en\").

    FILTER(CONTAINS(?attractionLabel, \"Yosemite \")).
    
}} ORDER BY DESC(?population) LIMIT 3 
            ").trim())
        ])
        .send()?;

    let body = response.text()?;
//    println!("{}", body);

    let solution: SolutionAttraction = serde_json::from_str(&body)?;
    Ok(solution.results.bindings)
}

fn find_artist(client: &Client, url: &str, headers: HeaderMap, location_id: &str) -> Result<Vec<BindingArtist>> {
    let response = client
        .get(url)
        .headers(headers)
        .query(&[
            ("format", "json"),
            ("query", format!("
PREFIX wd: <http://www.wikidata.org/entity/>
PREFIX wdt: <http://www.wikidata.org/prop/direct/>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
PREFIX schema: <http://schema.org>

SELECT DISTINCT ?artist ?artistLabel ?followers WHERE {{
    ?artist wdt:P136 ?genre;
            wdt:P8687 ?followers;
            rdfs:label ?artistLabel.
    FILTER(LANG(?artistLabel) = \"en\").

    ?artist wdt:P740 wd:{}

}} ORDER BY DESC(?followers) LIMIT 3
            ", location_id).trim())
        ])
        .send()?;

    let body = response.text()?;
//    println!("{}", body);

    let solution: SolutionArtist = serde_json::from_str(&body)?;
    Ok(solution.results.bindings)
}

fn find_subject(client: &Client, url: &str, headers: HeaderMap, location_id: &str) -> Result<Vec<BindingSubject>> {
    let response = client
        .get(url)
        .headers(headers)
        .query(&[
            ("format", "json"),
            ("query", format!("
PREFIX wd: <http://www.wikidata.org/entity/>
PREFIX wdt: <http://www.wikidata.org/prop/direct/>
PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
PREFIX schema: <http://schema.org>

SELECT DISTINCT ?subject ?subjectLabel ?boxOffice WHERE {{
    ?subject wdt:P31 wd:Q11424;
             wdt:P2142 ?boxOffice;
             rdfs:label ?subjectLabel.

    ?subject wdt:P840 wd:{}

    FILTER(LANG(?subjectLabel) = \"en\").

}} ORDER BY DESC(?boxOffice) LIMIT 3
            ", location_id).trim())
        ])
        .send()?;

    let body = response.text()?;
//    println!("{}", body);

    let solution: SolutionSubject = serde_json::from_str(&body)?;
    Ok(solution.results.bindings)
}

fn main() -> Result<()> {
    let client = Client::new();
    let url = "https://query.wikidata.org/sparql";
    let user_agent = "User-Agent: Other";

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, user_agent.parse().unwrap());

    let attractions = find_attraction(&client, url, headers.clone())?;

    for attraction in attractions.iter() {
        println!(
            "\"{}\", \"{}\", \"{:?}\", \"{}\", \"{}\", {}",
             attraction.attraction.get_local_name(),
             attraction.attraction_label.value,
             attraction.description,
             attraction.location.get_local_name(),
             attraction.location_label.value,
             attraction.population.get_number()
        );

        let location_id = &attraction.location.get_local_name();

        let artists = find_artist(&client, url, headers.clone(), location_id)?;
        for artist in artists {
            println!(
                "\"{}\", \"{}\", {}",
                artist.artist.get_local_name(),
                artist.artist_label.value,
                artist.followers.get_number()
            );
        }

        let subjects = find_subject(&client, url, headers.clone(), location_id)?;
        for subject in subjects {
            println!(
                "\"{}\", \"{}\", {}",
                subject.subject.get_local_name(),
                subject.subject_label.value,
                subject.box_office.get_number()
            );
        }
    }

    Ok(())
}
