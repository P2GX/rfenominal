# Rust




## Test drive

Fenominal depends on the [ontolius](https://docs.rs/ontolius/latest/ontolius/) crate
for ontology-related operations. To instantiate an Ontolius ontology object, we
require a copy of the `hp.json` file, which can be downloaded from the
[HPO website](https://hpo.jax.org/data/ontology).

Use the following code to initialize the Fenominal object.

```rust
use std::sync::Arc;
use fenominal::Fenominal;
use ontolius::io::OntologyLoaderBuilder;
use ontolius::ontology::csr::FullCsrOntology;
et hp_path = "some/path/hp.json";
let loader = OntologyLoaderBuilder::new().obographs_parser().build();
// replace unwrap with appropriate error handling
let hpo: FullCsrOntology = loader.load_from_path(json_path).unwrap(); 
let hpo_arc = Arc::new(hpo);
let fenomi
```

## process
Fenominal can process arbitarily long texts and return a list of `FenominalHit` objects:

```rust
let text = "Intellectual disability, macrocephaly, scoliosis";
let hits: Vec<FenominalHit> = fenominal.process(&text).unwrap();
```

These are defined as follows
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[serde(tag = "kind", rename_all = "camelCase")]
pub struct FenominalHit {
    /// The entity's term ID.
    pub term_id: String,
    /// The entity's label.
    pub label: String,
    /// The coordinates of the entity within the source text.
    pub span: Range<usize>,
    /// The observation status (present/excluded).
    pub is_observed: bool,
}
```

## GUIs

The other functionalities are designed to support use of fenominal in graphic user interfaces. This function returns a list of `FenominalSentence objects`:


```rust
let text = "Intellectual disability, macrocephaly, scoliosis...(multiple sentences)";
let hits: Vec<FenominalSentence> = fenominal.mine_sentences(&text).unwrap();
```

See the code for further documentation.

### Autocomplete
The fenominal library also offers autocomplete functionality (that has nothing to do with the Term-BLAST algorithm) that can be useful in GUIs.

```rust
let autocompleter = let autocompleter = AutoCompleter::new(hpo::clone())
let query = "Arach"; // user is entering the first letters of Arachnodactyly 
let limit = 20; // show up to the best 20 matches
let matches: Vec<OntologyMatch>  = autocompleter.search_hpo(query, line);
```

This yields of list of `OntologyMatch` objects:

```rust

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
```

## Command-line application 
Fenominal includes a small command-line application that demonstrates some of the capabilities.

```shell
 cargo run --bin fenominal_main  -- --hp /path/hp.json --input "intellectual disability (IQ 65), macrocephaly and dysmorphisms"
```
This will show the identified HPO terms.
