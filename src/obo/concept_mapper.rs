use std::collections::{HashMap, HashSet};
use super::{concept::Concept};

pub struct ConceptMapper {
    component_token_to_concept_map: HashMap<String, Vec<Concept>>,
}

impl ConceptMapper {
    pub fn new() -> Self {
        let map: HashMap<String, Vec<Concept>> = HashMap::new();
        ConceptMapper {
            component_token_to_concept_map: map,
        }
    }

    pub fn get_match(&self, words: &[&str]) -> std::option::Option<Concept> {
        let token_set: HashSet<String> = words.iter().map(|&s| s.to_string()).collect();
        for token in &token_set {
            let concept_list = self.component_token_to_concept_map.get(token);
            match concept_list {
                Some(clist) => {
                    for cpt in clist {
                        if cpt.non_stop_set_equal(&token_set) {
                            // We have a match!
                            return Some(cpt.clone());
                        }
                    }
                }
                _ => {}
            }
        }
        None // if we get here, we have not matched anything
    }

    pub fn add_concept(&mut self, concept: &Concept) {
        for token in concept.get_non_stop_words() {
            // insert a default value (empty vector) if the key is not present, then add the concept to the list
            self.component_token_to_concept_map
                .entry(token.clone())
                .or_insert(Vec::new())
                .push(concept.clone());
        }
    }

}

