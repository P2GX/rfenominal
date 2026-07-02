mod common;

use std::sync::Arc;

use ontolius::ontology::csr::FullCsrOntology;
use fenominal::{AutoCompleter, OntologyMatch};
use rstest::rstest;
use common::hpo;




#[rstest]
fn test_macrocephaly_autocomplete(
    hpo: Arc<FullCsrOntology>
) {
    let text="macroceph"; // user is searching for Macrocephaly and has entered this so far
    let autocompleter = AutoCompleter::new(hpo);
    let hits_limit = 20;
    let hits: Vec<OntologyMatch> = autocompleter.search_hpo(&text, hits_limit);
    let expected_hpo_id = "HP:0000256";
    let mut found = false;
    for hit in hits {
        if hit.id == expected_hpo_id {
            found = true;
        }
    }
    assert!(found);
   
}