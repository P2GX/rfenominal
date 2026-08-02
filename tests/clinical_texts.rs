mod common;

use std::sync::Arc;

use ontolius::ontology::csr::FullCsrOntology;
use fenominal::{Fenominal, FenominalHit};
use rstest::rstest;
use common::hpo;


const SENTENCE_1: &str = r#"An 8-week-old female twin born at 36 weeks had a history of frequent  diarrhea  and  emesis , with feeds resulting in failure to thrive (FTT)."#;




#[rstest]
fn test_ftt(
    hpo: Arc<FullCsrOntology>
) {
    let text="Failure to thrive";
    let fenominal = Fenominal::new_hpo(hpo);
    let fenominal_hits: Vec<FenominalHit> = fenominal.process(&text).unwrap();
    assert_eq!(1, fenominal_hits.len());
    let fhit = &fenominal_hits[0];
    assert_eq!("Failure to thrive", fhit.label);
}

#[rstest]
fn test_parse_para_1(
    hpo: Arc<FullCsrOntology>
) {
    let fenominal = Fenominal::new_hpo(hpo);
    let sanitized = fenominal::sanitize(SENTENCE_1);
    let fenominal_hits: Vec<FenominalHit> = fenominal.process(&sanitized).unwrap();
    assert_eq!(3, fenominal_hits.len());
}


fn assert_hit(hits: &[FenominalHit], term_id: &str, label: &str) {
    assert!(
        hits.iter().any(|h| {
            h.term_id == term_id && h.label == label 
        }),
        "expected hit {} [{}] not found in {:#?}",
        label, term_id,  hits
    );
}


const CASE_REPORT_1: &str = r#"
Physical examination revealed short stature (149 Cm), low set ears and ptosis.
There was high arched palate and pectus excavatum.
The electrocardiogram revealed ST segment depression.
Echocardiography showed hypertrophic cardiomyopathy and pulmonic stenosis."#;


// We should have four sentences with the following boundaries
// 0) 0 - 78 "Physical examination revealed short stature (149 Cm), low set ears and ptosis.""
// 1) 79- 129 "There was high arched palate and pectus excavatum."
// 2) 130- 183 ""The electrocardiogram revealed ST segment depression."
// 3) 184- "Echocardiography showed hypertrophic cardiomyopathy and pulmonic stenosis.""
#[rstest]
fn test_sentence_miner_1(
    hpo: Arc<FullCsrOntology>
) {
    let fenominal = Fenominal::new_hpo(hpo);
    let sentences = fenominal.mine_sentences(CASE_REPORT_1).unwrap();
    assert_eq!(
        sentences.len(), 4,
        "expected 4 sentences, got: {:#?}", sentences
    );
     let s0 = &sentences[0];
    assert_eq!(s0.start, 0);
    assert_eq!(
        s0.original_text,
        "Physical examination revealed short stature (149 Cm), low set ears and ptosis."
    );
    assert_eq!(s0.text_length(), 78);
    let hits = s0.hits();
    assert_hit(&hits, "HP:0004322", "Short stature");
    assert_hit(&hits, "HP:0000508", "Ptosis");
    assert_hit(&hits, "HP:0000369", "Low-set ears");
     // --- Sentence 1 ----------------------------------------------------
    let s1 = &sentences[1];
    assert_eq!(s1.start, 79);
    assert_eq!(
        s1.original_text,
        "There was high arched palate and pectus excavatum."
    );
    assert_eq!(s1.text_length(), 50);
    let hits = s1.hits();
    assert_hit(&hits, "HP:0000767", "Pectus excavatum");
    assert_hit(&hits, "HP:0000218", "High palate");
     // --- Sentence 2 ----------------------------------------------------
    let s2 = &sentences[2];
  
    assert_eq!(s2.start, 130);
    assert_eq!(
        s2.original_text,
        "The electrocardiogram revealed ST segment depression."
    );
    let hits = s2.hits();
    assert_eq!(s2.text_length(), 53);
    assert_hit(&hits, "HP:0012250", "ST segment depression");
    // --- Sentence 3 ----------------------------------------------------
    let s3 = &sentences[3];
    assert_eq!(s3.start, 184);
    assert_eq!(
        s3.original_text,
        "Echocardiography showed hypertrophic cardiomyopathy and pulmonic stenosis."
    );
    assert_eq!(s3.text_length(), 74);
    assert_hit(&s3.hits(), "HP:0001639", "Hypertrophic cardiomyopathy");
    assert_hit(&s3.hits(), "HP:0001642", "Pulmonic stenosis");
}




const CASE_REPORT_2: &str = r#"
Cough. Fever. Brachydactyly and Scoliosis. Not Short stature. Hypodontia"#;

// We should have four sentences with the following boundaries
// 0) 0- 6 "Cough."
// 1) 7-13 "Fever."
// 2) 14-42"Brachydactyly and Scoliosis."
// 3) 43-61 "Not Short stature."
// 4) 62-72 "Hypodontia"
#[rstest]
fn test_sentence_miner_2(hpo: Arc<FullCsrOntology>) {
    let fenominal = Fenominal::new_hpo(hpo);
    let sentences = fenominal.mine_sentences(CASE_REPORT_2).unwrap();

    assert_eq!(
        sentences.len(), 5,
        "expected 5 sentences, got: {:#?}", sentences
    );

    // --- Sentence 0: "Cough." -------------------------------------------
    let s0 = &sentences[0];
    assert_eq!(s0.start, 0);
    assert_eq!(s0.original_text, "Cough.");
    assert_eq!(s0.text_length(), 6);
    assert_hit(&s0.hits(), "HP:0012735", "Cough");

    // --- Sentence 1: "Fever." -------------------------------------------
    let s1 = &sentences[1];
    assert_eq!(s1.start, 7);
    assert_eq!(s1.original_text, "Fever.");
    assert_eq!(s1.text_length(), 6);
    assert_hit(&s1.hits(), "HP:0001945", "Fever");

    // --- Sentence 2: "Brachydactyly and Scoliosis." ----------------------
    let s2 = &sentences[2];
    assert_eq!(s2.start, 14);
    assert_eq!(s2.original_text, "Brachydactyly and Scoliosis.");
    assert_eq!(s2.text_length(), 28);
    assert_hit(&s2.hits(), "HP:0001156", "Brachydactyly");
    assert_hit(&s2.hits(), "HP:0002650", "Scoliosis");

    // --- Sentence 3: "Not Short stature." --------------------------------
    // This is the interesting one: "Not" should flip is_observed to false.
    let s3 = &sentences[3];
    assert_eq!(s3.start, 43);
    assert_eq!(s3.original_text, "Not Short stature.");
    assert_eq!(s3.text_length(), 18);
    assert_hit(&s3.hits(), "HP:0004322", "Short stature");

    // --- Sentence 4: "Hypodontia" (no terminal punctuation) --------------
    let s4 = &sentences[4];
    assert_eq!(s4.start, 62);
    assert_eq!(s4.original_text, "Hypodontia");
    assert_eq!(s4.text_length(), 10);
    assert_hit(&s4.hits(), "HP:0000668", "Hypodontia");
}
