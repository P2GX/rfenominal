use crate::{FenominalHit, FenominalSegment, FenominalSentence, util::error::FenominalError};




/// Converts a raw input string and a list of structural hits into a sequence of
/// displayable text annotations, ensuring safe handling of Unicode character boundaries.
///
/// This function iterates through a list of `fenominal_hits` (which contain byte spans)
/// and segments the `input_text` into two types of annotations:
/// 1. Non-hit (plain) text segments, which are HTML-escaped.
/// 2. Matched hit segments, which contain associated metadata.
///
/// **Safety & Error Handling:**
///
/// Due to the nature of UTF-8 strings in Rust, slicing (`&str[start..end]`)
/// requires that the provided indices (`start` and `end`) fall exactly on
/// valid character boundaries. Indices derived from external systems (like
/// text mining tools) may sometimes be misaligned, causing a runtime panic.
///
/// To prevent crashes, this function includes a safety guard:
/// - If a hit's span exceeds the text length or is otherwise invalid, it is skipped.
/// - **Crucially, if a hit's span indices do not align with valid UTF-8 character
///   boundaries in the `input_text`, the hit is safely skipped and logged to stderr.**
///   The function continues processing the remaining text and hits, maintaining
///   the last valid index (`last_index`).
///
/// # Arguments
///
/// * `input_text`: The full, raw UTF-8 string to be annotated.
/// * `fenominal_hits`: A slice of `FenominalHit` structures, where each hit
///   contains a byte span (`hit.span.start` and `hit.span.end`) relative to
///   the `input_text`.
///
/// # Returns
///
/// A `Result` containing:
/// * `Ok(Vec<TextAnnotationDto>)`: A vector of successfully generated text annotations.
/// * `Err(String)`: An error string, though the function is primarily designed
///   to skip errors and continue (error return paths are limited to unhandled
///   internal logic failures).
///
/// # Panics
///
/// This function is designed not to panic, but relies on `html_escape::encode_text`
/// and the internal logic of `TextAnnotationDto` and `FenominalHit` being correct.
pub fn fenominal_hits_to_sentence(
    input_text: &str,
    start_pos: usize,
    fenominal_hits: &[FenominalHit],
) -> Result<FenominalSentence, FenominalError> {
    let mut text_segments: Vec<FenominalSegment> = Vec::new();
    let mut last_index = 0usize;

    for hit in fenominal_hits {
        // hit.span is always document-absolute (map_sentence adds
        // start_pos_offset to every char position) -- translate to
        // sentence-local offsets before validating/slicing input_text.
        let (start, end) = match (
            hit.span.start.checked_sub(start_pos),
            hit.span.end.checked_sub(start_pos),
        ) {
            (Some(s), Some(e)) => (s, e),
            _ => {
                eprintln!("Skipping hit with span before sentence start: {:?}", hit);
                continue;
            }
        };

        if start >= end {
            return Err(FenominalError::invalid_span(hit.span.clone(), input_text.len(), input_text));
        }
        if end > input_text.len() {
            return Err(FenominalError::invalid_span(hit.span.clone(), input_text.len(), input_text));
        }
        if !input_text.is_char_boundary(start) || !input_text.is_char_boundary(end) {
            return Err(FenominalError::invalid_span(hit.span.clone(), input_text.len(), input_text));
        }

        if start > last_index {
            text_segments.push(FenominalSegment::plain_text(
                &input_text[last_index..start],
                start_pos + last_index,
            ));
        }

        let matched_text = &input_text[start..end];
        text_segments.push(FenominalSegment::from_hit(hit, matched_text));

        last_index = end;
    }

    if last_index < input_text.len() {
        text_segments.push(FenominalSegment::plain_text(
            &input_text[last_index..],
            start_pos + last_index,
        ));
    }

    Ok(FenominalSentence::new(start_pos, input_text, text_segments))
}