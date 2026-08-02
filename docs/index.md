---
layout: home

hero:
  name: "Fenominal"
  text: "HPO-based text mining engine"
  tagline: High-performance entity recognition in Rust with Python bindings
  actions:
    - theme: brand
      text: Get Started
      link: /guide/getting-started
    - theme: alt
      text: Python API
      link: /guide/python-api
    - theme: alt
      text: Rust API
      link: /guide/rust-api

features:
  - title: High Performance (Rust Core)
    details: Built in Rust for fast and memory-safe processing of large-scale clinical text corpora.
    link: /guide/rust-api

  - title: Python Integration
    details: Seamless bindings for use in data science and NLP pipelines via PyO3/Maturin.
    link: /guide/python-api

  - title: Ontology-Aware (HPO)
    details: Designed around the Human Phenotype Ontology for structured biomedical concept recognition.
    link: /guide/getting-started
---

## Overview

Fenominal is a high-performance **Human Phenotype Ontology (HPO)** text mining engine designed for robust named entity recognition in clinical and biomedical text.

It implements and extends the algorithm described in:

> Groza T, et al. (2023)  
> *Term-BLAST-like alignment tool for concept recognition in noisy clinical texts*  
> Bioinformatics 39(12): btad716  
> https://pubmed.ncbi.nlm.nih.gov/38001031/

---

## Key Capabilities

- Extracts HPO terms from noisy clinical narratives
- Optimized Rust backend for performance-critical workloads
- Python bindings for ML/NLP pipelines
- Designed for reproducible biomedical text mining

---

## Get Started

Choose your environment:

- 👉 **Rust users:** [Rust API Guide](./guide/rust-api)
- 🐍 **Python users:** [Python API Guide](./guide/python-api)
- 🚀 **New users:** [Getting Started](./guide/getting-started)