use super::Expansion;

static ASCII_LETTERS: [&'static str; 52] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L",
    "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
];
static ASCII_DIGITS: [&'static str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

pub enum CharRange {
    Digit,
    NonZeroDigit,
    Letters,
}
pub fn range_chars_as_str<'l_use>(type_of_range: CharRange) -> Vec<Expansion<'l_use>> {
    match type_of_range {
        CharRange::Digit => {
            let mut res = Vec::with_capacity(ASCII_DIGITS.len());
            for st in ASCII_DIGITS {
                res.push(super::Union::OnlyA(st.to_string()));
            }
            return res;
        },
        CharRange::NonZeroDigit => {
            let mut res = Vec::with_capacity(ASCII_DIGITS.len());
            for st in &ASCII_DIGITS[1..] {
                res.push(super::Union::OnlyA(st.to_string()));
            }
            return res;
        }
        CharRange::Letters => {
            let mut res = Vec::with_capacity(ASCII_LETTERS.len());
            for st in ASCII_LETTERS {
                res.push(super::Union::OnlyA(st.to_string()));
            }
            return res;
        }
    }
}

pub fn replace_scope_with_indent(target: &String) -> String {
    let mut result = target.clone();
    let mut tab_count = 0;
    let mut added_offset = 0;
    for (idx, ch) in target.chars().enumerate() {
        if ch == '{' {
            tab_count += 1;
        } else if ch == '}' {
            tab_count -= 1;
        } else if ch == '\n' {
            for _ in 0..tab_count {
                result.insert(idx + 1 + added_offset, ' ');
                added_offset += 1;
            }
        }
    }
    result = result.replace("{", "");
    result.replace("}", "")
}