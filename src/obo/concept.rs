use std::collections::HashSet;

use ontolius::TermId;

use crate::stopwords::is_stop;

/// Represent a concept from the Ontology (label or synonym) in which the non-stop words are placed in a set
///
///  This class contains one HPO label or synonym and provides functions for
/// searching for matches in input texts. For instance, we store all of the non-stop words in a set and can
/// check not only for exact matches with the label etc but permutations thereof.
/// This object is intended to represent a concept from the Ontology and not a hit in an actual text.
#[derive(Clone, Debug)]
pub struct Concept {
    original_concept: String,
    non_stop_words: HashSet<String>,
    term_id: TermId,
}

impl Concept {
    pub fn new(concept: &str, tid: TermId) -> Self {
        let words: Vec<&str> = concept.split_whitespace().collect();
        let filtered_words: HashSet<String> = words
            .into_iter()
            .filter(|word| !is_stop(word))
            .map(|word| word.to_string())
            .collect();
        Concept {
            original_concept: concept.into(),
            non_stop_words: filtered_words,
            term_id: tid,
        }
    }

    pub fn get_non_stop_words(&self) -> &HashSet<String> {
        return &self.non_stop_words;
    }

    pub fn non_stop_set_equal(&self, other_non_stop_words: &HashSet<String>) -> bool {
        return self.non_stop_words == *other_non_stop_words;
    }

    pub fn get_term_id(&self) -> &TermId {
        &self.term_id
    }

    pub fn term_id_equal(&self, other_tid: &TermId) -> bool {
        self.term_id == *other_tid
    }

    pub fn word_count(&self) -> usize {
        self.non_stop_words.len()
    }

    /// We will use the presence or absence of commas to decide 'ties' between matches that are equally
    ///long. If a match does not have a comma, we will consider it is a better match.
    pub fn has_comma(&self) -> bool {
        self.original_concept.contains(",")
    }
}

#[cfg(test)]
mod test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test() {
        // Cone-shaped epiphysis of the proximal phalanx of the 3rd finger HP:0009348
        let term_id: TermId = ("HP", "0009348").into();
        let term_label = "Cone-shaped epiphysis of the proximal phalanx of the 3rd finger";
        let hconcept = Concept::new(term_label, term_id);
        assert_eq!(term_label, hconcept.original_concept);
        // We have 8 words but only 6 non-stop words ("of" and "the" are stop words)
        assert_eq!(6, hconcept.word_count());
        let nstops: HashSet<String> = vec![
            "Cone-shaped",
            "epiphysis",
            "proximal",
            "phalanx",
            "3rd",
            "finger",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
        assert!(hconcept.non_stop_set_equal(&nstops));
        assert!(!hconcept.has_comma());
        //let tid = hconcept.get_hpo_id();
        //assert_eq!("HP", tid.)
    }
}
