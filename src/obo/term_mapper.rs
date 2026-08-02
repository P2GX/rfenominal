use std::{collections::HashMap, sync::Arc};

use ontolius::{
    ontology::{HierarchyWalks, OntologyTerms},
    term::{MinimalTerm, Synonymous},
    TermId,
};


use crate::models::ontology_profile::OntologyProfile;

use super::{
    concept::Concept, concept_mapper::ConceptMapper,
    ontology_loader::get_text_to_term_map,
};

pub struct TermMapper {
    wordcount_to_matcher: HashMap<usize, ConceptMapper>,
    max_tokens: usize
}

impl TermMapper {
    /// Fallback max-token-count used only if `text_to_term_id` is empty.
   /// Otherwise the real max is computed dynamically from the loaded terms.
    pub const DEFAULT_MAX_TOKEN_COUNT: usize = 14;

    pub fn new<O, T>(
        ontology: Arc<O>,
        profile: &OntologyProfile) -> Self
    where
        O: OntologyTerms<T> + HierarchyWalks,
        T: MinimalTerm + Synonymous,
    {
        let text_to_term_map = get_text_to_term_map(ontology, profile);
        TermMapper::from_map(text_to_term_map.iter().map(|(k, v)| (k.as_ref(), v)))
    }

    /// Create an TermMapper from text_to_tid_map
    ///
    /// # Arguments
    ///
    /// * `text_to_term_id` - An iterator with mapping from text to corresponding term ID.
    ///
    /// # Returns
    ///
    /// An TermMapper object that is ready to use for text mining.
    pub fn from_map<'a, I>(text_to_term_id: I) -> Self
    where
        I: IntoIterator<Item = (&'a str, &'a TermId)>,
    {
        let concepts: Vec<_> = text_to_term_id
            .into_iter()
            .map(|(k, v)| Concept::new(k, v.clone()))
            .collect();

        let max_tokens = concepts.iter().map(|c| c.word_count()).max().unwrap_or(TermMapper::DEFAULT_MAX_TOKEN_COUNT);

        let mut wc_map: HashMap<usize, ConceptMapper> = HashMap::new();
        for i in 1..=max_tokens {
            wc_map.insert(i, ConceptMapper::new());
        }
        for concept in concepts {
            let n = concept.word_count();
            wc_map.get_mut(&n).unwrap().add_concept(&concept);
        }
        TermMapper {
            wordcount_to_matcher: wc_map,
            max_tokens
        }
    }

    /// Search for a term that matches an input string.
    ///
    /// # Arguments
    ///
    /// * `tokens` - A listt of tokens (words) representing the input string
    ///
    /// # Returns
    /// A `Concept` if a match was found, or `None` otherwise.
    pub fn get_match(&self, tokens: &[&str]) -> Option<Concept> {
        if tokens.len() > self.max_tokens {
            println!("Malformed input. Slice length too large: {}", tokens.len());
            return None;
        } else if tokens.is_empty() {
            return None;
        } else {
            let matcher = self.wordcount_to_matcher.get(&tokens.len())?;
            return matcher.get_match(tokens);
        }
    }

    pub fn max_tokens(&self) -> usize {
        self.max_tokens
    }

    pub fn for_hpo<O, T>(ontology: Arc<O>) -> Self
    where
        O: OntologyTerms<T> + HierarchyWalks,
        T: MinimalTerm + Synonymous,
    {
        Self::new(ontology, &OntologyProfile::hpo())
    }

    pub fn for_maxo<O, T>(ontology: Arc<O>) -> Self
    where
        O: OntologyTerms<T> + HierarchyWalks,
        T: MinimalTerm + Synonymous,
    {
        Self::new(ontology, &OntologyProfile::maxo())
    }

}
