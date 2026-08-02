use std::{collections::{HashMap, HashSet}, sync::Arc};

use ontolius::{
    ontology::{HierarchyWalks, OntologyTerms},
    term::{MinimalTerm, Synonymous},
    TermId,
};

use crate::models::ontology_profile::OntologyProfile;


pub fn get_text_to_term_map<O, T>(
    ontology: Arc<O>,
    profile: &OntologyProfile,
) -> HashMap<String, TermId>
where
    O: OntologyTerms<T> + HierarchyWalks,
    T: MinimalTerm + Synonymous,
{
    let mut text_to_tid_map = HashMap::new();
    // These are commmon false-positive results related to HPO synonyms that occur in other contexts
    let omittable_labels: HashSet<String> = ["negative", "weakness"]
        .iter()
        .map(|s| s.to_ascii_lowercase())
        .collect();
    let min_synonym_length = 4;

    for term in ontology
        .iter_descendant_ids(&profile.root_term_id)
        .flat_map(|term_id| ontology.term_by_id(term_id))
    {
        let term_id = term.identifier();
        let term_label_lc = term.name().to_ascii_lowercase();
        if omittable_labels.contains(&term_label_lc) || term_label_lc.len() < min_synonym_length {
            continue;
        }
        text_to_tid_map.insert(term_label_lc, term_id.clone());
        for synonym in term.synonyms() {
            if omittable_labels.contains(&synonym.name) || synonym.name.len() < min_synonym_length {
                continue;
            }
            text_to_tid_map.insert(synonym.name.to_ascii_lowercase(), term_id.clone());
        }
    }

    text_to_tid_map
}


