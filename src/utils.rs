

/// Replaces all occurrences of `find` with `replace` in `s`. Returns a bool for if replacements were made.
pub fn replace(s: &str, find: &str, replace: &str) -> (bool, String) {
    let mut out = String::new();
    let find_bytes = find.as_bytes();
    let find_len = find_bytes.len();
    let mut changed = false;

    let mut i = 0;
    while i + find_len <= s.len() {
        if &s[i..i+find_len] == find {
            out.push_str(replace);
            i += find_len;
            changed = true;
        } else {
            out.push(s.chars().nth(i).unwrap());
            i += 1;
        }
    }
    (changed, out)
}