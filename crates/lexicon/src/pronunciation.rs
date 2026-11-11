use crate::Numeral;
use crate::NumeralCodec;
use crate::Onset;

/// Normalization recipe for declared spellings, read from the first word of a
/// multiword or hyphenated form: a leading sign is spoken *plus* or *minus*
/// (*a +1/+1 counter*, *a -1/-1 counter*), digits as a cardinal, an uppercase
/// initialism by its first letter name, and declared silent-`h` and
/// consonantal-vowel prefixes override the first letter. Otherwise the first
/// ASCII letter decides; an unknown onset stays `None`. Declared overrides
/// take precedence, and the grammar reads only the resulting [`Onset`],
/// never the spelling.
pub(crate) fn default_onset(surface: &str) -> Option<Onset> {
    if let Some(onset) = surface.chars().next().and_then(mark_onset) {
        return Some(onset);
    }
    let word = surface
        .split(|c: char| c.is_whitespace() || c == '-')
        .next()?;
    if word.is_empty() {
        return None;
    }
    let digits: String = word
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == ',')
        .collect();
    if !digits.is_empty() {
        return digits.replace(',', "").parse().ok().and_then(numeral_onset);
    }
    let letters: String = word.chars().take_while(char::is_ascii_alphabetic).collect();
    if !letters.is_empty() && letters.chars().all(|c| c.is_ascii_uppercase()) {
        return Some(
            if matches!(
                letters.as_bytes()[0],
                b'A' | b'E' | b'F' | b'H' | b'I' | b'L' | b'M' | b'N' | b'O' | b'R' | b'S' | b'X'
            ) {
                Onset::Vowel
            } else {
                Onset::Consonant
            },
        );
    }
    let lower = word.to_ascii_lowercase();
    if ["heir", "honest", "honor", "hour"]
        .iter()
        .any(|p| lower.starts_with(p))
    {
        return Some(Onset::Vowel);
    }
    if lower.starts_with("eu")
        || lower == "one"
        || lower.starts_with("once")
        || [
            "unit",
            "unite",
            "unity",
            "unicorn",
            "uniform",
            "unique",
            "union",
            "universe",
            "universal",
            "university",
            "use",
            "user",
            "usual",
            "utensil",
            "utility",
            "utopia",
        ]
        .iter()
        .any(|p| lower.starts_with(p))
    {
        return Some(Onset::Consonant);
    }
    match lower.as_bytes().first().copied() {
        Some(b'a' | b'e' | b'i' | b'o' | b'u') => Some(Onset::Vowel),
        Some(c) if c.is_ascii_alphabetic() => Some(Onset::Consonant),
        _ => None,
    }
}

/// The Onset of a mark spoken as a word: a sign, *plus* or *minus* (*a
/// +8/+8 creature*). Any other mark is not spoken and has none.
#[must_use]
pub fn mark_onset(mark: char) -> Option<Onset> {
    matches!(mark, '+' | '-').then_some(Onset::Consonant)
}

/// The Onset of an integer, spoken as a cardinal (*an 8/8*, *a 1/1*).
#[must_use]
pub fn numeral_onset(value: i32) -> Option<Onset> {
    default_onset(&Numeral::Cardinal.format(value))
}
