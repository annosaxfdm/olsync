#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;

// OLS API *******************
#[derive(Deserialize, Debug)]
enum Status {
    LOADED,
    SKIP,
    FAILED,
    LOADING,
}

#[derive(Deserialize, Debug)]
enum ReasonerType {
    EL,
    OWL2,
    NONE,
}

#[derive(Deserialize, Debug)]
struct Annotations {
 license: Option<Vec<String>>,
 creator: Option<Vec<String>>,
 rights: Option<Vec<String>>,
 #[serde(alias = "format-version")]
 formatversion: Option<Vec<String>>,
 comment: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
struct OntologyConfig {
    annotations: Annotations,
    id: String,
    versionIri: Option<String>,
    title: Option<String>,
    namespace: String,
    preferredPrefix: String,
    description: Option<String>,
    homepage: Option<String>,
    fileLocation: String,
    oboSlims: bool,
    reasonerType: ReasonerType,
    baseUris: Vec<String>,
    labelProperty: String,
    synonymProperties: Vec<String>,
    hierarchicalProperties: Vec<String>,
    hiddenProperties: Vec<String>,
    internalMetadataProperties: Vec<String>,
    skos: bool,
}

#[derive(Deserialize, Debug)]
struct Ontology {
    ontologyId: String,
    loaded: Option<String>,
    updated: Option<String>,
    status: Status,
    message: String,
    version: Option<String>,
    fileHash: String,
    loadAttempts: u32,
    config: OntologyConfig,
}

#[derive(Deserialize, Debug)]
struct Embedded {
    ontologies: Vec<Ontology>,
}

#[derive(Deserialize, Debug)]
struct Href {
    href: String,
}

#[derive(Deserialize, Debug)]
struct Links {
    next: Option<Href>,
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
    uri: String,
    ontology_purl: String,
    title: Option<String>,
    preferredPrefix: Option<String>,
    description: Option<String>,
    base_uri: Vec<String>,
    homepage: Option<String>,
    //mailing_list: Option<String>,
    //definition_property: Option<String>,
    synonym_property: Vec<String>,
    hierarchical_property: Vec<String>,
    hidden_property: Vec<String>,
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
        uri: o.config.id.clone(),
        description: o.config.description.clone(),
        homepage: o.config.homepage.clone(),
        base_uri: o.config.baseUris.clone(),
        //definition_property: ..
        synonym_property: o.config.synonymProperties.clone(),
        hierarchical_property: o.config.hierarchicalProperties.clone(),
        hidden_property: o.config.hiddenProperties.clone(),
    }
}

fn transform(embedded: &Embedded) -> OlsConfig {
    let mut len = embedded.ontologies.len();
    if let Ok(maxs) = env::var("OLSYNC_MAX_ONTOLOGIES") {
        if let Ok(max) = maxs.parse::<usize>() {
            len = core::cmp::min(len, max);
        }
    }
    OlsConfig {
        ontologies: embedded.ontologies[..len].iter().map(transformO).collect(),
    }
}

/* */
fn load(url: &str) -> Result<OntologiesRoot, reqwest::Error> {
    let mut root: OntologiesRoot = reqwest::blocking::get(url)?.json()?;

    let mut cursor: &OntologiesRoot = &root;
    let mut nextRoot: OntologiesRoot;
    while let Some(ref nextRef) = cursor._links.next {
        println!("{}", nextRef.href);
        nextRoot = reqwest::blocking::get(nextRef.href.clone())?.json()?;
        root._embedded
            .ontologies
            .append(&mut nextRoot._embedded.ontologies);
        cursor = &nextRoot;
    }
    //println!("{:#?}",root);
    for o in &root._embedded.ontologies {
        //println!("{:#?}", o)
    }
    Ok(root)
}

fn loads(urls: &Vec<String>) -> Result<Embedded, reqwest::Error> {
    let it = urls
        .iter()
        .map(|u| load(&(u.to_owned() + &"ontologies".to_owned())));
    // prevent duplicates
    let mut map = HashMap::new();
    for r in it {
        for ontology in r?._embedded.ontologies {
            map.insert(ontology.ontologyId.clone(), ontology);
        }
    }
    Ok(Embedded {
        ontologies: map.into_values().collect(),
    })
}

fn save(ols: OlsConfig, filename: &str) -> Result<(), Box<dyn Error>> {
    let s = serde_yaml::to_string(&ols)?;
    fs::write(filename, s)?;
    println!(
        "{} ontologies written to {}",
        ols.ontologies.len(),
        filename
    );
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let DEFAULT_URIS =
        "https://terminology.nfdi4chem.de/ts/api/ https://terminology.nfdi4ing.de/ts4ing/api/"
            .to_owned();
    let uris = env::var("OLSYNC_API_URLS")
        .unwrap_or(DEFAULT_URIS)
        .split_whitespace()
        .map(String::from)
        .collect();
    let embedded = loads(&uris)?;
    save(
        transform(&embedded),
        &env::var("OLSYNC_CONFIG_FILE").unwrap_or("olsync.yml".to_string()),
    )?;
    Ok(())
}
