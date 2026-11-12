use crate::Numeral;
use crate::NumeralCodec;
use crate::Onset;
use crate::SurfaceStructure;

/// The Onset of a declared spelling, from its first word, where its Lexeme
/// declares none ([`crate::Lexeme::onset`]):
///
/// - a word that begins with an ASCII letter: `Vowel` for *a*, *e*, *i*, *o* or
///   *u*, else `Consonant`. Every word whose first sound is not its first
///   letter's (*hour*, *unity*, *X*) declares its Onset.
/// - a numeral: the Onset of its spoken cardinal ([`numeral_onset`]: *2ˣ*).
/// - a measured compound: the Onset of its measure's first component, a signed
///   one spoken with its sign, *plus* or *minus* (*a +1/+1 counter*), and an
///   unsigned one as its cardinal.
///
/// Any other spelling has no Onset, which no article agrees with.
pub(crate) fn default_onset(surface: &str, structure: SurfaceStructure) -> Option<Onset> {
    if structure == SurfaceStructure::MeasuredCompound
        && let Some(word) = SIGN_WORDS
            .iter()
            .find(|(sign, _)| surface.starts_with(*sign))
            .map(|(_, word)| word)
    {
        return letter_onset(word);
    }
    let digits: String = surface
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == ',')
        .collect();
    if !digits.is_empty() {
        return digits.replace(',', "").parse().ok().and_then(numeral_onset);
    }
    letter_onset(surface)
}

/// The words the signs are spoken as: *plus*, and *minus* for the
/// hyphen-minus and the minus sign (U+2212) alike.
const SIGN_WORDS: [(char, &str); 3] = [('+', "plus"), ('-', "minus"), ('−', "minus")];

/// The Onset of a sign, spoken as its word (*a +8/+8 creature*); `None` for
/// a mark no sign word speaks.
#[must_use]
pub fn sign_onset(mark: char) -> Option<Onset> {
    SIGN_WORDS
        .iter()
        .find(|(sign, _)| *sign == mark)
        .and_then(|(_, word)| letter_onset(word))
}

/// The Onset of a word by its first letter, if that is an ASCII letter.
fn letter_onset(word: &str) -> Option<Onset> {
    let first = word.chars().next()?;
    first.is_ascii_alphabetic().then(|| {
        if matches!(first.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u') {
            Onset::Vowel
        } else {
            Onset::Consonant
        }
    })
}

/// The cardinal words ([`Numeral::Cardinal`]) whose Onset is not their first
/// letter's: *one* begins with /w/ (*a 1/1*).
const CARDINAL_ONSETS: [(&str, Onset); 1] = [("one", Onset::Consonant)];

/// The Onset of an integer, spoken as a cardinal (*an 8/8*, *a 1/1*): its
/// first word's.
#[must_use]
pub fn numeral_onset(value: i32) -> Option<Onset> {
    let cardinal = Numeral::Cardinal.format(value);
    let word = cardinal.split([' ', '-']).next()?;
    CARDINAL_ONSETS
        .iter()
        .find(|(declared, _)| *declared == word)
        .map(|(_, onset)| *onset)
        .or_else(|| letter_onset(word))
}
