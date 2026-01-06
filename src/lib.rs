//! Depot GBK - Character encoding detection library
//!
//! This library provides functionality for detecting character encoding in byte sequences,
//! with a focus on Chinese character encoding detection.

/// Result of character encoding detection
#[derive(Debug, Clone)]
pub struct EncodingResult {
    pub charset: String,
    pub confidence: f32,
    pub language: String,
    pub is_valid_utf8: bool,
}

/// Detect the character encoding of the given byte sequence
///
/// # Arguments
///
/// * `data` - The byte sequence to analyze
///
/// # Returns
///
/// Returns an `EncodingResult` containing the detected charset, confidence level,
/// language, and UTF-8 validity information.
///
/// # Examples
///
/// ```
/// use depot_gbk_rust::detect_encoding_detailed;
///
/// let text = "这是一段测试文本。";
/// let result = detect_encoding_detailed(text.as_bytes());
/// println!("Charset: {}, Confidence: {:.2}%", result.charset, result.confidence * 100.0);
/// ```
pub fn detect_encoding_detailed(data: &[u8]) -> EncodingResult {
    let (charset, confidence, language) = chardet::detect(data);
    let is_valid_utf8 = std::str::from_utf8(data).is_ok();

    EncodingResult {
        charset,
        confidence,
        language,
        is_valid_utf8,
    }
}

/// Simple encoding detection that returns just the charset name
///
/// # Arguments
///
/// * `data` - The byte sequence to analyze
///
/// # Returns
///
/// Returns the detected charset as a String.
pub fn detect_charset(data: &[u8]) -> String {
    let (charset, _, _) = chardet::detect(data);
    charset
}

/// Check if the given byte sequence is valid UTF-8
///
/// # Arguments
///
/// * `data` - The byte sequence to check
///
/// # Returns
///
/// Returns `true` if the data is valid UTF-8, `false` otherwise.
pub fn is_utf8(data: &[u8]) -> bool {
    std::str::from_utf8(data).is_ok()
}

/// Check if the given byte sequence is likely to be GBK encoded Chinese text
///
/// This is a simple heuristic that checks for common GBK byte patterns.
///
/// # Arguments
///
/// * `data` - The byte sequence to check
///
/// # Returns
///
/// Returns `true` if the data appears to be GBK encoded, `false` otherwise.
pub fn is_likely_gbk(data: &[u8]) -> bool {
    if data.is_empty() {
        return false;
    }

    // Simple heuristic: check for GBK byte patterns
    // GBK uses byte ranges: A1-FE for the first byte, A1-FE for the second byte
    let mut gbk_like_pairs = 0;
    let mut total_pairs = 0;

    let mut i = 0;
    while i < data.len() - 1 {
        let first = data[i];
        let second = data[i + 1];

        // Check if this looks like a GBK character pair
        if (0xA1..=0xFE).contains(&first) && (0xA1..=0xFE).contains(&second) {
            gbk_like_pairs += 1;
        }
        total_pairs += 1;
        i += 2;
    }

    if total_pairs == 0 {
        return false;
    }

    // If more than 30% of byte pairs look like GBK, consider it likely GBK
    (gbk_like_pairs as f32 / total_pairs as f32) > 0.3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_encoding_utf8() {
        let text = "这是一段测试文本。"; // Chinese text will be detected as UTF-8
        let result = detect_encoding_detailed(text.as_bytes());
        assert_eq!(result.charset, "utf-8");
        assert!(result.is_valid_utf8);
    }

    #[test]
    fn test_detect_charset() {
        let text = "这是一段测试文本。"; // Chinese text will be detected as UTF-8
        let charset = detect_charset(text.as_bytes());
        assert_eq!(charset, "utf-8");
    }

    #[test]
    fn test_is_utf8() {
        let text = "Hello, 世界!";
        assert!(is_utf8(text.as_bytes()));

        // Invalid UTF-8 sequence
        let invalid_utf8 = [0xff, 0xfe];
        assert!(!is_utf8(&invalid_utf8));
    }

    #[test]
    fn test_is_likely_gbk() {
        // Empty data should return false
        assert!(!is_likely_gbk(&[]));

        // ASCII text should not be detected as GBK
        let ascii = "Hello, world!";
        assert!(!is_likely_gbk(ascii.as_bytes()));
    }
}