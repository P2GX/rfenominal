# Python



## Installation

fenominal is not yet in PyPI. For now, follow the following instructions

1. Clone the package, cd into the package with an active virtual environment, and install fenominal
```bash
git clone https://github.com/P2GX/fenominal.git
cd fenominal
python3 -m venv venv
source venv/bin/activate
pip install maturin
maturin develop
```


### Python

Fenominal can be run like this:

```bash
import json
from fenominal import Fenominal

# Point this to a valid HPO JSON file path on your system
fe = Fenominal("path/to/hp.json")
result = fe.map_text("Patient presents with severe microcephaly.")
print(json.loads(result))
```
