# Python



## Installation

fenominal can be installed from [PyPI](https://pypi.org/project/fenominal/) using `pip`.

```bash
pip install fenominal
```


## Using fenominal

Fenominal can be run like this:

```python
import json
from fenominal import Fenominal

# Point this to a valid HPO JSON file path on your system
fenominal = Fenominal("path/to/hp.json")
result = fenominal.map_text("Patient presents with severe microcephaly.")
print(json.loads(result))
```

The output of the command is a list of dictionaries with this structure:

```json
[
  {
    "term_id": "string (ontology term ID — HP:xxxxxxx or MAXO:xxxxxxx, depending on which ontology was loaded)",
    "label": "string (Ontology term name)",
    "span": {
      "start": "integer (Character start index)",
      "end": "integer (Character end index)"
    },
    "is_observed": "boolean (Presence of concept referred to by Ontology term)"
  }
]
```

This corresponds to the `FenominalHit` struct of the [Rust fenominal library](rust-api.md).

## MAxO

fenominal defaults to HPO, but can be used for [Medical Action Ontology (MAxO)](https://www.ebi.ac.uk/ols4/ontologies/maxo) by
adding a second argument of simply ``"maxo"``.

```python
from fenominal import Fenominal

hpo_miner = Fenominal("/some/path/hp.json")    
maxo_miner = Fenominal("/some/path/maxo.json", "maxo")

hpo_hits = json.loads(hpo_miner.map_text("Intellectual disability, macrocephaly"))
maxo_hits = json.loads(maxo_miner.map_text("cardiac catheterization, lymphadenectomy"))

print(hpo_hits)
print(maxo_hits)
```
