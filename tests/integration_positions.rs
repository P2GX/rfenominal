mod common;

use std::sync::Arc;

use ontolius::{ontology::{OntologyTerms, csr::FullCsrOntology}, term::{MinimalTerm}};
use fenominal::{Fenominal, FenominalHit};
use rstest::rstest;
use common::hpo;


const PARA1: &str = r#"This patient was a 2-year-old boy, the second child of nonconsanguineous parents of German and Dutch/Polish origin, respectively. The further family history was unremarkable. Pregnancy was complicated by fetal hydronephrosis and bilateral dilated ureter. The boy was delivered spontaneously at 40 weeks of gestation with a length of 53 cm (mean), weight of 4,510 g (+2.0 SD), and occipitofrontal head circumference (OFC) of 38.5 cm (+2.2 SD). The Apgar scores were 8, 10, and 10 at 1, 5, and 10 min, respectively, and the umbilical arterial cord pH of 7.19 was normal. Physical examination showed a median cleft palate which had been surgically corrected in the first months of life. Abdominal ultrasound confirmed bilateral dilated ureter and showed in addition ureteral ectopia requiring surgical therapy."#;

/// We expect hydronephrosis	HP:0000126
/// median cleft palate	HP:0009099

#[rstest]
fn test_parse_para_1(
    hpo: Arc<FullCsrOntology>
) {
    let fenominal = Fenominal::new_hpo(hpo);
    let fenominal_hits: Vec<FenominalHit> = fenominal.process(PARA1).unwrap();
    let hydronephrosis_start = PARA1.find("hydronephrosis").unwrap();
    let hydrouterer_start = PARA1.find("dilated ureter").unwrap();
    let mcp_start = PARA1.find("median cleft palate").unwrap();

    /*
     for h in &fenominal_hits {
        println!("{:?}",h);
    }
    Expect:
    FenominalHit { term_id: "HP:0000126", label: "Hydronephrosis", span: 210..224, is_observed: true }
    FenominalHit { term_id: "HP:0000072", label: "Hydroureter", span: 239..253, is_observed: true }
    FenominalHit { term_id: "HP:0009099", label: "Median cleft palate", span: 599..618, is_observed: true }
    FenominalHit { term_id: "HP:0000072", label: "Hydroureter", span: 725..739, is_observed: true }
    */
    assert_eq!(4, fenominal_hits.len());
    let hit1 = fenominal_hits.get(0).unwrap();
    let hit2 = fenominal_hits.get(1).unwrap();
    let hit3 = fenominal_hits.get(2).unwrap();
    
    assert_eq!(hydronephrosis_start, hit1.span.start);
    assert_eq!(hydrouterer_start, hit2.span.start);
    assert_eq!(mcp_start, hit3.span.start);
}


#[rstest]
fn test_failure_to_thrive_exists_in_hpo( hpo: Arc<FullCsrOntology>) {
    // for some reason, Failure to thrive does not git picked up by fenominal
    // This is a sanity check that the term exists in the ontology file we 
    // are using for testing.
    let ftt_label = "Failure to thrive";
    let found = hpo.iter_terms()
        .map(MinimalTerm::name)
        .any(|t| t == ftt_label);
    assert!(found);
}

#[rstest]
fn test_median_cp(
    hpo: Arc<FullCsrOntology>
) {
    // Expect to find Cleft palate HP:0000175
    let text = "Physical examination showed a cleft palate which had been surgically corrected";
    let hpo_arc = hpo.clone();
    let fenominal = Fenominal::new_hpo(hpo_arc);
    let fenominal_hits: Vec<FenominalHit> = fenominal.process(text).unwrap();
    assert_eq!(1, fenominal_hits.len());
    let cp = fenominal_hits[0].clone();
    assert_eq!("Cleft palate", cp.label);
}