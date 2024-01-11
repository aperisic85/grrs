//std::out expect bytes so we need to use std::io::Write (u8) instead std::fmt::Write.
pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            if let _res = writeln!(writer, "{}", line) {}
        }
    }
}
