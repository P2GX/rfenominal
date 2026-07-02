//! AutoCompleter
//! Functionality to support autocompletion with HPO terms
//! //! ```
//! use std::fs::File;
//! use std::io::BufReader;
//! use std::sync::Arc;
//! use flate2::bufread::GzDecoder;
//! use fenominal::AutoCompleter;
//! use fenominal::HpoMatch;
//! use ontolius::io::OntologyLoaderBuilder;
//! use ontolius::ontology::csr::FullCsrOntology;
//! let hp_path = "resources/hp.v2025-03-03.json.gz";
//! let loader = OntologyLoaderBuilder::new().obographs_parser().build();
//! let hpo: FullCsrOntology = loader.load_from_read(
//!              GzDecoder::new(BufReader::new(File::open(hp_path).expect("HPO should be readable")))
//!            ).expect("HPO should be well formatted");
//! let hpo = Arc::new(hpo);
//! let autocompleter = AutoCompleter::new(hpo);
//! 
//!
//! // Perform autocompletion. The user might be searching for "Macrocephaly"
//! // in the front-end and might have entered "macroc" so far
//! // See tests/autocomplete_test.rs
//! let text = "macroceph";
//! let hits: Vec<HpoMatch> = autocompleter.search_hpo(&text);
//! ```

use std::sync::Arc;
use std::sync::OnceLock;
use ontolius::ontology::{HierarchyWalks, OntologyTerms};
use ontolius::common::hpo::PHENOTYPIC_ABNORMALITY;
use ontolius::term::{MinimalTerm, Synonymous};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use regex::Regex;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};



#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OntologyMatch {
    /// HPO identifier of the matched concept, e.g., HP:0011995
    pub id: String,
    /// Corresponding HPO label, e.g., Atrial septal dilatation 
    pub label: String,
    /// Text that was matched, e.g., Atrial septal aneurysm
    pub matched_text: String,
}


pub struct AutoCompleter {
    /// Strings for autocompletion
    hpo_auto_complete: Vec<OntologyMatch>,
}

impl AutoCompleter {
    /// Create a new AutoCompleter object
    /// Can be initializyed aith Arc<FullCsrOntology> (smart pointer to HPO Ontolius object)
    pub fn new<O, T>(hpo: Arc<O>) -> Self where
        O: OntologyTerms<T> + HierarchyWalks,
        T: MinimalTerm + Synonymous, {
        let acomplete = Self::initialize_hpo_autocomplete(hpo.clone());
        Self {
            hpo_auto_complete: acomplete
        }
    }
    

    /// Set up autocomplete functionality 
    fn initialize_hpo_autocomplete<O, T>(hpo: Arc<O>) -> Vec<OntologyMatch> where
        O: OntologyTerms<T> + HierarchyWalks,
        T: MinimalTerm + Synonymous, {
        let mut hpo_auto_complete: Vec<OntologyMatch> = Vec::new();
        for tid in  hpo.iter_descendant_ids(&PHENOTYPIC_ABNORMALITY) {
            match hpo.term_by_id(tid) {
                Some(term) => {
                    let id_str = tid.to_string();
                    let primary_label = term.name().to_string();
                    hpo_auto_complete.push(OntologyMatch {
                        id: id_str.clone(),
                        label: primary_label.clone(),
                        matched_text: primary_label.clone(),
                    });
                    for synonym in term.synonyms() {
                        let label = synonym.name.clone();
                        hpo_auto_complete.push(OntologyMatch {
                            id: id_str.clone(),
                            label: primary_label.clone(),
                            matched_text: label, 
                        });
                    }
                },
                None => { eprintln!("Could not retrieve term for {}", tid); } // should never happen
            }
        }
        hpo_auto_complete
    }

     /// Provide Strings with TermId - Label that will be used for autocompletion
    pub fn search_hpo(&self, query: &str, limit: usize) -> Vec<OntologyMatch> {
        let matcher = SkimMatcherV2::default();
        let query_lower = query.to_lowercase();
        static HPO_ID_REGEX: OnceLock<Regex> = OnceLock::new();
        let re = HPO_ID_REGEX.get_or_init(|| Regex::new(r"HP:\d{7}").unwrap());
        if let Some(mat) = re.find(query) {
            let exact_id = mat.as_str();
            if let Some(exact_match) = self.hpo_auto_complete.iter().find(|item| item.id == exact_id) {
                return vec![exact_match.clone()];
            }
        }
        // get fuzzy matches to query
        let mut matches: Vec<_> = self.hpo_auto_complete
            .iter()
            .filter_map(|item| {
                matcher.fuzzy_match(&item.matched_text, &query_lower)
                    .map(|score| (score, item))
            })
            .collect();
        // sort by score
        matches.sort_unstable_by_key(|&(score, _)| std::cmp::Reverse(score));
        // return best hits, but only one hit per HPO id (avoid duplicates because of synonym matches)
        let mut seen = std::collections::HashSet::new();
        matches
            .into_iter()
            .map(|(_, item)| item)
            .filter(|item| seen.insert(&item.id))
            .take(limit)
            .cloned()
            .collect()
    }

     /// We want to get the single best match of any HPO term label to the query string
    pub fn get_best_hpo_match(&self, query: String) -> Option<OntologyMatch> {
        let matcher = SkimMatcherV2::default();
        let query_lower = query.to_lowercase();
        // First, prioritize exact matches
        let exact_match = self.hpo_auto_complete
            .iter()
            .find(|item| item.matched_text.to_lowercase() == query_lower);

        if let Some(item) = exact_match {
            return Some(item.clone());
        }
        // Otherwise, try to get a good fuzzy match
        self.hpo_auto_complete
            .iter()
            .filter_map(|item| {
                // We score based on the matched_text (could be a synonym or primary label)
                matcher.fuzzy_match(&item.matched_text, &query)
                    .map(|score| (item, score))
            })
            // Get the highest scoring match
            .max_by_key(|(_, score)| *score)
            // Return the whole object so you have the ID and Label immediately
            .map(|(item, _)| item.clone())
    }
           

}


