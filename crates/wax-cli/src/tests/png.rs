//! PNG header parsing, plus the fixture builder other test modules reuse.

use crate::png_dimensions;
use std::io::Write;
use tempfile::NamedTempFile;

/// Build the first 24 bytes of a PNG: signature, IHDR length, "IHDR", and the
/// width/height fields. That is exactly what `png_dimensions` reads, so the
/// file does not need to be a decodable image.
pub(crate) fn png_header(width: u32, height: u32) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(24);
    bytes.extend_from_slice(b"\x89PNG\r\n\x1a\n");
    bytes.extend_from_slice(&13u32.to_be_bytes()); // IHDR data length
    bytes.extend_from_slice(b"IHDR");
    bytes.extend_from_slice(&width.to_be_bytes());
    bytes.extend_from_slice(&height.to_be_bytes());
    bytes
}

fn write_temp(content: &[u8]) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(content).unwrap();
    file.flush().unwrap();
    file
}

fn path_of(file: &NamedTempFile) -> &str {
    file.path().to_str().unwrap()
}

#[test]
fn reads_width_and_height() {
    let file = write_temp(&png_header(1920, 1080));
    assert_eq!(png_dimensions(path_of(&file)), Some((1920, 1080)));
}

#[test]
fn reads_zero_and_large_dimensions() {
    for (w, h) in [(0u32, 0u32), (1, 1), (7680, 4320), (u32::MAX, u32::MAX)] {
        let file = write_temp(&png_header(w, h));
        assert_eq!(png_dimensions(path_of(&file)), Some((w, h)));
    }
}

#[test]
fn rejects_a_wrong_signature() {
    let mut bytes = png_header(10, 10);
    bytes[1] = b'X';
    let file = write_temp(&bytes);
    assert_eq!(png_dimensions(path_of(&file)), None);
}

#[test]
fn rejects_a_header_that_is_not_ihdr() {
    let mut bytes = png_header(10, 10);
    bytes[15] = b'X';
    let file = write_temp(&bytes);
    assert_eq!(png_dimensions(path_of(&file)), None);
}

#[test]
fn rejects_a_file_shorter_than_the_header() {
    for len in [0usize, 1, 8, 16, 23] {
        let file = write_temp(&png_header(1, 1)[..len]);
        assert_eq!(png_dimensions(path_of(&file)), None, "len {}", len);
    }
}

#[test]
fn rejects_a_missing_file() {
    assert_eq!(png_dimensions("/nonexistent/definitely/not/here.png"), None);
}

#[test]
fn rejects_arbitrary_binary_content() {
    let file = write_temp(&[0xFFu8; 64]);
    assert_eq!(png_dimensions(path_of(&file)), None);
}
