#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
enum Status {
    LOADED,
    SKIP,
}

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
    config: OntologyConfig,
}

#[derive(Deserialize, Debug)]
struct Embedded {
    ontologies: Vec<Ontology>,
}

#[derive(Deserialize, Debug)]
struct OntologiesRoot {
    _embedded: Embedded,
}

#[derive(Serialize, Debug)]
struct OboOntology {
    //activity_status: String
    id: String,
    ontology_purl: String,
    title: String,
    preferredPrefix: Option<String>,
}

#[derive(Serialize, Debug)]
struct OboConfig {
    ontologies: Vec<OboOntology>,
}

fn transform(embedded: &Embedded) -> OboConfig {
    OboConfig {ontologies: Vec::new()}
}

fn load(url: &str) -> Result<OntologiesRoot, reqwest::Error> {
    let root: OntologiesRoot = reqwest::blocking::get(url)?.json()?;
    // TODO: read following pages
    //print!("{:#?}",root);
    for o in &root._embedded.ontologies {
        print!("{:#?}", o)
    }
    Ok(root)
}

fn save(x: OboConfig, filename: &str) -> Result<(), reqwest::Error> {
    //print!("{}",serde_yaml::to_string(&embedded));
    Ok(())
}

fn main() -> Result<(), reqwest::Error> {
    let uri = "https://terminology.nfdi4ing.de/ts4ing/api/ontologies";
    //let uri = "https://www.ebi.ac.uk/ols/api/ontologies/";
    let root = load(uri)?;
    //save(root._embedded,"/tmp/test.yml")?;
    Ok(())
}
