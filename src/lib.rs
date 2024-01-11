//std::out expect bytes so we need to use std::io::Write (u8) instead std::fmt::Write.
pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            if let _res = writeln!(writer, "{}", line) {}
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    crate::find_matches(
        "where are\nyou now baby\nblah Mercedes benz",
        "where",
        &mut result,
    );
    assert_eq!(result, b"where are\n"); //fn find_matches() returns whole line so wee need to compare whole line not pattern only
}