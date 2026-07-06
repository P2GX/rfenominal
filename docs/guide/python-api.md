# Python



## Installation

fenominal can be installed from [PyPI](https://pypi.org/project/fenominal/) using `pip`.

```bash
pip install fenominal
```


## Using fenominal

Fenominal can be run like this:

```bash
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
    "term_id": "string (HPO Term ID)",
    "label": "string (Phenotype name)",
    "span": {
      "start": "integer (Character start index)",
      "end": "integer (Character end index)"
    },
    "is_observed": "boolean (Presence of phenotypic feature)"
  }
]
```

This corresponds to the `FenominalHit` struct of the [Rust fenominal library](rust-api.md).

