//! This module contains code to support autocomplete matching 
//! It can be used in front-end tools

/// This represents a match for an HPO term to a potentially partial text
/// that is being entered by a user 
/// 

pub mod autocompleter;

pub use self::autocompleter::{AutoCompleter, OntologyMatch};

