use std::str::FromStr;

use ontolius::TermId;

/// Fenominal can be used for several different OBO ontologies.
/// The primary use case is Human Phenotype Ontology (HPO), 
/// but we have implemented parsing for Medical Action Ontology (MAxO) as well.
pub struct OntologyProfile {
    pub name: &'static str,     // "HPO", "MAXO", "MONDO" — for error messages/logging
    pub root_term_id: TermId,   // e.g. HP:0000118, MAXO:0000001, MONDO:0700096
}

impl OntologyProfile {
    pub fn hpo() -> Self {
        Self { name: "HPO", root_term_id: TermId::from_str("HP:0000118").unwrap() }
    }
    pub fn maxo() -> Self {
        Self { name: "MAXO", root_term_id: TermId::from_str("MAXO:0000001").unwrap() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}