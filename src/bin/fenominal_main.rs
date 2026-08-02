use clap::Parser;
use ontolius::io::OntologyLoaderBuilder;
use ontolius::ontology::csr::FullCsrOntology;
use fenominal::Fenominal;
use fenominal::FenominalHit;
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(version = "0.1.6", about = "Fenominal implementation in Rust")]
struct Args {
    /// Path to the file
    #[arg(long, value_name = "FILE")]
    hp: PathBuf,

    /// Input string
    #[arg(short, long, value_name = "STRING")]
    input: String,
}

fn main() -> Result<(), Box<dyn Error>>{
    let args = Args::parse();
    let hp_json_path = args.hp;
    let hp_json_path_str: &str = hp_json_path.to_str().expect("Invalid UTF-8 in path");
    let input_string = args.input;
    let hpo_path = Path::new(hp_json_path_str);
    if hpo_path.exists() {
        println!("Processing HPO JSON file: {:?}.", hp_json_path);
    } else {
        return Err(format!("Could not find HPO JSON file at {}.", hp_json_path_str).into());
    }
    println!("[INFO] Input string: {}", input_string);
    let loader = OntologyLoaderBuilder::new().obographs_parser().build();
    let hpo: FullCsrOntology = loader.load_from_path(hp_json_path_str).unwrap();
    let hpo = Arc::new(hpo);
    let fenominal = Fenominal::new_hpo(hpo);
    let fenominal_hits: Vec<FenominalHit> = fenominal.process(&input_string)?;
    
    // pretty-print the JSON response
    let pretty_fenominal_hits = serde_json::to_string_pretty(&fenominal_hits).unwrap();
    println!("[INFO] Hits:\n{}", &pretty_fenominal_hits);
    Ok(())
}
