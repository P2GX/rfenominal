
use std::sync::Arc;


use crate::core_document::CoreDocument;
use crate::hpo::sentence_mapper::SentenceMapper;
use crate::hpo::text_to_annotation::fenominal_hits_to_sentence;
use crate::models::fenominal_model::{FenominalHit, FenominalSentence};
use crate::simple_sentence::SimpleSentence;
use crate::util::error::FenominalError;
use crate::{sanitize, sentence_split};
use ontolius::io::OntologyLoaderBuilder;
use ontolius::ontology::csr::FullCsrOntology;
use ontolius::ontology::{HierarchyWalks, OntologyTerms};
use ontolius::term::{MinimalTerm, Synonymous};
use ontolius::term::simple::SimpleTerm;

#[cfg(feature = "serde")]




/// Fenominal text mining.
pub struct Fenominal<O, T> where
        O: OntologyTerms<T> + HierarchyWalks,
        T: MinimalTerm + Synonymous  {
    sentence_mapper: SentenceMapper<O,T>,
}

impl<O, T> Fenominal<O, T> 
    where
    O: OntologyTerms<T> + HierarchyWalks,
    T: MinimalTerm + Synonymous  
    {

    pub fn new(hpo: Arc<O>)-> Self {
        let hpo_arc = Arc::clone(&hpo);
        Self {
            sentence_mapper: SentenceMapper::new(hpo_arc),
        }
    }


    pub fn map_text(&self, text: &str) -> Result<Vec<FenominalHit>, FenominalError> {
        let core_document = CoreDocument::new(text);
        let sentences = core_document.get_sentences();
        let mut mapped_parts: Vec<FenominalHit> = Vec::new();
        for ss in sentences {
            let sentence_parts = self.sentence_mapper.map_sentence(ss)?;
            mapped_parts.extend(sentence_parts);
        }
        Ok(mapped_parts)
    }

    pub fn process(
        &self, 
        text: &str) -> Result<Vec<FenominalHit>, FenominalError> {
        self.map_text(text)
    }

    fn mine_sentence(&self, sentence: &str,  start: usize) -> Result<FenominalSentence, FenominalError> {
        let sentence_end = start + sentence.len() - 1;
        let ss = SimpleSentence::new(sentence, start, sentence_end);
        let hits =  self.sentence_mapper.map_sentence(&ss)?;
        fenominal_hits_to_sentence(sentence,start,  &hits)
          
    }

    pub fn mine_sentences(&self, text: &str) -> Result<Vec<FenominalSentence>, FenominalError> {
        let sanitized_text = sanitize(text);
        let sentences = sentence_split(&sanitized_text);
        let mut start = 0 as usize;
        let mut fenom_sent_list = Vec::with_capacity(sentences.len());
        for s in sentences.into_iter() {
            let fsent = self.mine_sentence(&s, start)?;
            start += fsent.text_length() +1;
            fenom_sent_list.push(fsent);
        }
        Ok(fenom_sent_list)
    }

}    



impl Fenominal<FullCsrOntology, SimpleTerm> {
    pub fn from_hpo_json(path: &str) -> Result<Self, String> {
        let loader = OntologyLoaderBuilder::new().obographs_parser().build();
        
        // If load_from_path can fail, it's safer to map the error rather than panicking with expect
        let ontology: FullCsrOntology = loader
            .load_from_path(path)
            .map_err(|e| format!("Could not load {}: {}", path, e))?;
            
        let hpo_arc = Arc::new(ontology);
        Ok(Self::new(hpo_arc))
    }
}
