use std::fmt;
use std::ops::Range;
use serde::{Deserialize, Serialize};

/// A sentence of the original text
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[serde(tag = "kind", rename_all = "camelCase")]
pub struct FenominalSentence { 
    /// Start offset of this sentence within the original source text.
    /// Units match `FenominalHit::span` (so hit spans and sentence
    /// start are directly comparable).
    pub start: usize,
    pub original_text: String,
    pub segments: Vec<FenominalSegment>
}

impl FenominalSentence {
    pub fn new(
        start: usize, 
        original: impl Into<String>, 
        segments: Vec<FenominalSegment>) -> Self {
        Self { 
            start, 
            original_text: original.into(),
            segments 
        }
    }

    pub fn segments(&self) -> &[FenominalSegment] {
        &self.segments
    }

    /// Only the matched HPO entities, excluding plain-text segments.
    pub fn hit_iter(&self) -> impl Iterator<Item = &FenominalHit> {
        self.segments.iter().filter_map(|s| match s {
            FenominalSegment::Hit(hit) => Some(&hit.hit),
            FenominalSegment::Text(_) => None,
        })
    }

    pub fn hits(&self) -> Vec<FenominalHit> {
        self.hit_iter().cloned().collect()
    }

    pub fn text_length(&self) -> usize {
        self.original_text.len()
    }
}

impl fmt::Display for FenominalSentence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Sentence (start @ {}):", self.start)?;
        for hit in &self.segments {
            writeln!(f, "  {}", hit)?;
        }
        Ok(())
    }
}

/// A named entity identified by text mining.
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

impl FenominalHit {
    pub fn new(term_id: String, label: &str, span: Range<usize>, is_observed: bool) -> Self {
        Self {
            term_id,
            label: label.to_string(),
            span,
            is_observed,
        }
    }

    /// get the start/end position of a 'Hit'
    pub fn get_span(&self) -> Range<usize> {
        Clone::clone(&self.span)
    }
}


impl fmt::Display for FenominalHit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} [{}] @ {}..{} ({})",
            self.label,
            self.term_id,
            self.span.start,
            self.span.end,
            if self.is_observed { "observed" } else { "excluded" }
        )
    }
}

/// Text from a sentence that was not parsed as a hit (i.e., "in-between")
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[serde(tag = "kind", rename_all = "camelCase")]
pub struct FenominalText {
    pub text: String,
    pub span: Range<usize>,
}


impl fmt::Display for FenominalText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)?;
        Ok(())  
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[serde(tag = "kind", rename_all = "camelCase")]
pub struct FenominalHitSegment {
    pub text: String,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub hit: FenominalHit,
}

impl fmt::Display for FenominalHitSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.hit)
    }
}


/// A contiguous piece of a sentence: either a recognized entity or plain text.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum FenominalSegment {
    Hit(FenominalHitSegment),
    Text(FenominalText),
}


impl fmt::Display for FenominalSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FenominalSegment::Hit(hit) => write!(f, "{}", hit),
            FenominalSegment::Text(text) => write!(f, "{}", text),
        }
    }
}


impl FenominalSegment {

    pub fn plain_text(text_segment: impl Into<String>, start_pos: usize) -> Self {
        let text: String = text_segment.into();
        let end_pos = start_pos + text.len();
        FenominalSegment::Text(
            FenominalText { 
                text, 
                span: start_pos..end_pos, 
            }
        )
    }

     pub fn from_hit(hit: &FenominalHit, matched_text: impl Into<String>) -> Self {
        FenominalSegment::Hit(
            FenominalHitSegment {
                text: matched_text.into(),
                hit: hit.clone()
            }
        )
    }
}