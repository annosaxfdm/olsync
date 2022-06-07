#![allow(non_snake_case)]
#![allow(dead_code)]

use serde::{Deserialize};

#[derive(Deserialize, Debug)]
enum Status {LOADED, SKIP}

#[derive(Deserialize, Debug)]
struct OntologyConfig {
    id: String,
    versionIri: Option<String>,
    namespace: String,
    preferredPrefix: String,
    title: String,
    fileLocation: String,
}

#[derive(Deserialize, Debug)]
struct Ontology {
    ontologyId: String,
    loaded: Option<String>,
    updated: Option<String>,
    status: Status,
    config: OntologyConfig
}

#[derive(Deserialize, Debug)]
struct Embedded {
    ontologies: Vec<Ontology>
}

#[derive(Deserialize, Debug)]
struct OntologiesRoot {
    _embedded: Embedded
}


fn load(url: &str) -> Result<(), reqwest::Error> {
    let root: OntologiesRoot = reqwest::blocking::get(url)?.json()?;
    //print!("{:#?}",root);
    for o in root._embedded.ontologies {
        print!("{:#?}",o)
    }
    Ok(())
}
fn main() -> Result<(), reqwest::Error> {
    load("https://www.ebi.ac.uk/ols/api/ontologies/")
}
