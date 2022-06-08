#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

// OLS API *******************
#[derive(Deserialize, Debug)]
enum Status {
    LOADED,
    SKIP,
    FAILED,
    LOADING
}

#[derive(Deserialize, Debug)]
struct OntologyConfig {
    id: String,
    versionIri: Option<String>,
    namespace: String,
    preferredPrefix: String,
    title: Option<String>,
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
struct Href{
    href: String
}

#[derive(Deserialize, Debug)]
struct Links {
    next: Option<Href>
}

#[derive(Deserialize, Debug)]
struct OntologiesRoot {
    _embedded: Embedded,
    _links: Links,
}

// OLS Config ************
#[derive(Serialize, Debug)]
struct OlsOntology {
    //activity_status: String
    id: String,
    ontology_purl: String,
    title: Option<String>,
    preferredPrefix: Option<String>,
}

#[derive(Serialize, Debug)]
struct OlsConfig {

    ontologies: Vec<OlsOntology>,
}
// ***************************

fn transformO(o: &Ontology) -> OlsOntology {
    OlsOntology {
        id: o.ontologyId.clone(),
        ontology_purl: o.config.fileLocation.clone(),
        title: o.config.title.clone(),
        preferredPrefix: Some(o.config.preferredPrefix.clone()),
    }
}

fn transform(embedded: &Embedded) -> OlsConfig {
    let max = core::cmp::min(999,embedded.ontologies.len()); // for debugging
    OlsConfig {
        ontologies: embedded.ontologies[..max].iter().map(transformO).collect(),
    }
}

fn load(url: &str) -> Result<OntologiesRoot, reqwest::Error> {
    let mut root: OntologiesRoot = reqwest::blocking::get(url)?.json()?;

    let mut cursor: &OntologiesRoot = &root;
    let mut nextRoot: OntologiesRoot;
    while let Some(ref nextRef) = cursor._links.next {
        println!("{}", nextRef.href);
        nextRoot = reqwest::blocking::get(nextRef.href.clone())?.json()?;
        root._embedded.ontologies.append(&mut nextRoot._embedded.ontologies);
        cursor = &nextRoot;
    }
    // TODO: read following pages
    //println!("{:#?}",root);
    for o in &root._embedded.ontologies {
        //println!("{:#?}", o)
    }
    Ok(root)
}

fn save(ols: OlsConfig, filename: &str) -> Result<(), Box<dyn Error>> {
    let s = serde_yaml::to_string(&ols)?;
    fs::write(filename, s)?;
    println!("{} ontologies written to {}", ols.ontologies.len(), filename);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    //let uri = "https://terminology.nfdi4ing.de/ts4ing/api/ontologies";
    let uri = "https://www.ebi.ac.uk/ols/api/ontologies/";
    let root = load(uri)?;
    save(transform(&root._embedded), "test.yml")?;
    Ok(())
}
