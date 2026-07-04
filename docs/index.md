---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "Fenominal"
  text: "HPO-based text mining in Rust and Python"
  tagline: My great project tagline
  actions:
    - theme: brand
      text: Markdown Examples
      link: /markdown-examples
    - theme: alt
      text: API Examples
      link: /api-examples


features:
  - title: Blazing Fast
    details: Leverages Rust's memory safety and speed to parse clinical texts instantly.
  - title: Python Bindings
    details: Fully interoperable with Python data science pipelines via PyO3/Maturin.
  - title: HPO Integrated
    details: Designed to be integrated with the Human Phenotype Ontology.
---

<div class="home-custom-content">
  This package is a Rust implementation of the 
  <a href="https://pubmed.ncbi.nlm.nih.gov/38001031/" target="_blank" rel="noopener noreferrer">fenominal</a> 
  algorithm, for which we initially created a 
  <a href="https://github.com/monarch-initiative/fenominal" target="_blank" rel="noopener noreferrer">Java</a> 
  implementation.
</div>

<style scoped>
.home-custom-content {
  max-width: 1152px;
  margin: 64px auto 32px auto;
  padding: 0 64px;
  font-size: 1.25rem;
  line-height: 1.6;
  text-align: center;
  color: var(--vp-c-text-1);
}

.home-custom-content a {
  color: var(--vp-c-brand-1);
  font-weight: 500;
  text-decoration: underline;
  text-underline-offset: 4px;
}

.home-custom-content a:hover {
  color: var(--vp-c-brand-2);
}

@media (max-width: 960px) {
  .home-custom-content {
    padding: 0 48px;
  }
}

@media (max-width: 640px) {
  .home-custom-content {
    padding: 0 24px;
    font-size: 1.1rem;
  }
}
</style>