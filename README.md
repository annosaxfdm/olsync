# OLS 3 Synchronizer

Given a source OLS 3 instance, creates an OLS YAML configuration file for importing the same ontologies into another OLS 3 instance.
Research prototype.

## Usage
Requires Rust and Cargo to be installed.
Clone the repository and `cargo run`.

## Environment Variables

| Variable                   | Description |
| ------------------------   | ------                   |
| OLSYNC\_API\_URLS          | Whitespace separated list of source OLS instance API URLs. Duplicated IDs will be ignored. |
| OLSYNC\_MAX\_ONTOLOGIES    | Limit the maximum amount of loaded ontologies for faster testing or with limited resources, however you can't control which ones get left out. When not specified, then all ontologies will be loaded.  |
| OLSYNC\_CONFIG\_FILE       | Path to the yaml file where the output is saved. Will be overwritten. Put this in either `obo-config.yaml` or `ols-config.yaml` in the `config` folder of your OLS installation, the format is the same. You can use the other configuration file to specify your own ontologies.

## Docker

### Setup
Integrate olsync into an OLS Docker Compose setup.
Below is an example from the anno branch of <https://github.com/annosaxfdm/ols> that olsync is cloned as a sibling directory.
Adding olsync as a Git submodule is more elegant but may incur frequent submodule update commits while olsync is still under development.

    services:
    
      solr:
        [...]
    
      mongo:
        [...]
     
      olsync:
        build: ../olsync
        environment:
          - OLSYNC_API_URLS=https://terminology.nfdi4chem.de/ts/api/ https://terminology.nfdi4ing.de/ts4ing/api/
          - OLSYNC_CONFIG_FILE=/app/obo-config.yaml
          - OLSYNC_MAX_ONTOLOGIES=3
        volumes:
            - olsync:/app
    
      ols-config-importer:
        [...]
        volumes:
          - olsync:/config
        depends_on:
          mongo:
        condition: service_started
          olsync:
        condition: service_completed_successfully
        restart: on-failure:2

### Docker Commands

* Run `docker compose build` after initial setup and each non-runtime change.
* Start the Docker Compose setup with `docker compose up`.
* Get new versions of the ontologies with `docker compose restart`, however this may not delete already existing ontologies don't exist in the new configuration files.
* To properly remove deleted ontologies, delete the volumes with `docker compose down -v` and then `docker compose up` again, however that may take much longer.

## Background and Motivation
There are different ways of sharing ontologies, and one such way is with terminology servers like [OntoPortal](https://ontoportal.org/) and [OLS](https://www.ebi.ac.uk/ols/ontologies).
Besides your own ontologies, it can be useful to also include ontologies from other OLS instances.
While OntoPortal already had a synchronization mechanism, this was not found for OLS.
Synchronization between ANNO and NFDI OLS instances is [one of the tasks of the ANNO project](https://annosaxfdm.de/workpackages/).

## FAQ

### What's the difference between obo-config.yaml and ols-config.yaml?
There seems to have been a historical difference, when the former was for ontologies serialized in the [OBO format](http://owlcollab.github.io/oboformat/doc/obo-syntax.html) and the latter was for ontologies serialized as OWL RDF/XML but in practice those seem to be interchangeable in the current state.

### Why is this a separate component and not part of OLS itself?
This could also be implemented as a modification of the [OLS config importer](https://github.com/EBISPOT/OLS/tree/dev/ols-apps/ols-config-importer) but we decided for a separate component because of the following:

* compact, easy to develop and maintain
* the API works great and is quite stable
* upstream updates don't need to be merged

### Should I use Docker or not?
If you just want to import the ontologies of some target OLS instances into your own a single time and then leave it like that, you don't need docker.
Just compile olsync with Rust and Cargo and run it.
If you need binaries, create [an issue](https://github.com/annosaxfdm/olsync/issues) and tell me which platform you are on and we will compile it for you.

However the typical use case for olsync is to run it automatically in regular intervals on a server so that if the set of ontologies in the source OLS instances changes, your own OLS instance gets updated as well.
This use case is solved with a Docker Compose setup, which is regularily restarted.
