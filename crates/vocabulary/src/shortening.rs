//! The shortened spelling each card's own text uses for it [CR#201.5c],
//! computed from the Oracle corpus as the card names load.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use sylex_lexicon::Numeral;
use sylex_lexicon::NumeralCodec;
use sylex_upstream::scryfall::OracleCard;
use sylex_upstream::scryfall::OracleCardReadError;
use sylex_upstream::scryfall::oracle_cards;

use crate::SourceError;

/// The shortened spelling each legendary permanent face's own Oracle text
/// uses for it, keyed by full card name.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AttestedShortenings {
    shortenings: BTreeMap<String, String>,
}

impl AttestedShortenings {
    /// Computes every face's attested shortening from the Oracle corpus.
    ///
    /// # Errors
    /// A corpus record that fails `OracleCardReader` validation, or a
    /// supported legendary face with no type line.
    pub fn from_corpus(
        cards: impl IntoIterator<Item = Result<OracleCard, OracleCardReadError>>,
    ) -> Result<Self, SourceError> {
        let mut titles = BTreeSet::new();
        let mut faces = Vec::new();
        for card in cards {
            let card = card.map_err(|error| SourceError::Invalid(error.to_string()))?;
            for unit in card.oracle_units() {
                titles.insert(unit.display_name().to_owned());
                if card.supported() {
                    let type_line = unit.type_line().ok_or_else(|| {
                        SourceError::Invalid(format!(
                            "supported face {} has no type line",
                            unit.display_name()
                        ))
                    })?;
                    if legendary_permanent(type_line) {
                        faces.push((
                            unit.display_name().to_owned(),
                            // A vanilla legendary has no Oracle text.
                            unit.oracle_text().unwrap_or("").to_owned(),
                        ));
                    }
                }
            }
        }
        let titles = title_index(&titles);
        let shortenings = faces
            .iter()
            .filter_map(|(name, text)| {
                attested_short_name(name, text, &titles)
                    .map(|short| (name.clone(), short.to_owned()))
            })
            .collect();
        Ok(Self { shortenings })
    }

    /// Computes the shortenings from the pinned Oracle corpus.
    ///
    /// # Errors
    /// An unreadable corpus, or any [`Self::from_corpus`] error.
    pub fn load() -> Result<Self, SourceError> {
        Self::from_corpus(
            oracle_cards().map_err(|error| SourceError::Invalid(error.to_string()))?,
        )
    }

    /// The shortened spelling `name`'s own text uses, if any.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&str> {
        self.shortenings.get(name).map(String::as_str)
    }
}

fn legendary_permanent(type_line: &str) -> bool {
    let head = type_line.split_once('—').map_or(type_line, |(head, _)| head);
    let words = head.split_whitespace().collect::<Vec<_>>();
    words.contains(&"Legendary")
        && words.iter().any(|word| {
            matches!(
                *word,
                "Artifact" | "Battle" | "Creature" | "Enchantment" | "Land" | "Planeswalker"
            )
        })
}

// Shortened self-reference has the full name's referent [CR#201.5c]. The
// truncation patterns propose candidates; the unit's Oracle text must attest
// the candidate outside an occurrence of any complete card title.
// The full and shortened spellings remain distinct variants of one Lexeme.
fn attested_short_name<'a>(name: &'a str, text: &str, titles: &TitleIndex<'_>) -> Option<&'a str> {
    let candidate = short_name(name)?;
    let candidate_spans = text
        .match_indices(candidate)
        .filter_map(|(start, _)| {
            let end = start + candidate.len();
            (!text[..start]
                .chars()
                .next_back()
                .is_some_and(name_word_character)
                && !text[end..].chars().next().is_some_and(name_word_character)
                && !text[start..].starts_with(name))
            .then_some(start..end)
        })
        .collect::<Vec<_>>();
    if candidate_spans.is_empty() {
        return None;
    }
    let first_word = name_words(candidate).next()?;
    let title_spans = titles
        .get(first_word)
        .into_iter()
        .flatten()
        .filter(|title| title.len() > candidate.len() && title.contains(candidate))
        .flat_map(|title| {
            text.match_indices(*title)
                .map(|(start, _)| start..start + title.len())
        })
        .filter(|span| {
            !text[..span.start]
                .chars()
                .next_back()
                .is_some_and(name_word_character)
                && !text[span.end..]
                    .chars()
                    .next()
                    .is_some_and(name_word_character)
        })
        .collect::<Vec<_>>();
    candidate_spans
        .iter()
        .any(|candidate_span| {
            !title_spans
                .iter()
                .any(|span| span.start <= candidate_span.start && span.end >= candidate_span.end)
        })
        .then_some(candidate)
}

fn name_word_character(character: char) -> bool {
    character.is_alphanumeric() || matches!(character, '_' | '-')
}

type TitleIndex<'a> = BTreeMap<&'a str, Vec<&'a str>>;

fn name_words(name: &str) -> impl Iterator<Item = &str> {
    name.split(|character: char| !character.is_alphanumeric())
        .filter(|word| !word.is_empty())
}

fn title_index(titles: &BTreeSet<String>) -> TitleIndex<'_> {
    let mut index: TitleIndex<'_> = BTreeMap::new();
    for title in titles {
        for word in name_words(title).collect::<BTreeSet<_>>() {
            index.entry(word).or_default().push(title);
        }
    }
    index
}

fn short_name(name: &str) -> Option<&str> {
    let candidate = if let Some((prefix, _)) = name.split_once(',') {
        prefix.trim()
    } else if name.starts_with("The ") {
        return None;
    } else if let Some((prefix, last)) = name.rsplit_once(' ')
        && Numeral::Roman.parse(last).is_ok()
    {
        prefix.trim_end()
    } else if let Some(index) = [" and the ", " the ", " of "]
        .into_iter()
        .filter_map(|separator| name.find(separator))
        .min()
    {
        &name[..index]
    } else {
        name.split(' ').next().unwrap_or(name)
    };
    (candidate != name && !candidate.is_empty()).then_some(candidate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oracle_short_names_preserve_multiword_prefixes_and_use_the_numeral_codec() {
        for (name, expected) in [
            ("Gut, True Soul Zealot", Some("Gut")),
            ("The Balrog, Durin's Bane", Some("The Balrog")),
            ("The First Sliver", None),
            ("King Darien XLVIII", Some("King Darien")),
            ("Grand Arbiter Augustin IV", Some("Grand Arbiter Augustin")),
            ("Tor Wauki the Younger", Some("Tor Wauki")),
            ("Sidar Jabari of Zhalfir", Some("Sidar Jabari")),
            ("Nissa Revane", Some("Nissa")),
            ("Shang-Chi and the Ten Rings", Some("Shang-Chi")),
            ("Progenitus", None),
        ] {
            assert_eq!(short_name(name), expected, "{name}");
        }
        assert!(legendary_permanent("Legendary Artifact Creature — Golem"));
        assert!(!legendary_permanent("Legendary Sorcery"));
        assert!(!legendary_permanent("Creature — Legendary"));
    }

    #[test]
    fn only_standalone_short_references_in_pinned_oracle_text_become_aliases() {
        let expected = BTreeMap::from([
            ("Captain Sisay", None),
            ("Shang-Chi and the Ten Rings", Some("Shang-Chi")),
            ("Captain Lannery Storm", None),
            ("Brothers Yamazaki", None),
            ("Gut, True Soul Zealot", None),
            ("Nissa Revane", None),
            ("Dina, Soul Steeper", Some("Dina")),
            ("Uurg, Spawn of Turg", Some("Uurg")),
            ("The Balrog, Durin's Bane", Some("The Balrog")),
            ("Tor Wauki the Younger", Some("Tor Wauki")),
            ("King Darien XLVIII", Some("King Darien")),
        ]);
        let mut titles = BTreeSet::new();
        let mut observed = BTreeMap::new();
        for card in oracle_cards().unwrap() {
            let card = card.unwrap();
            for unit in card.oracle_units() {
                let name = unit.display_name();
                titles.insert(name.to_owned());
                if card.supported() && expected.contains_key(name) {
                    let text = unit.oracle_text().unwrap();
                    observed.insert(name.to_owned(), text.to_owned());
                }
            }
        }
        assert_eq!(observed.len(), expected.len());
        let aliases = AttestedShortenings::load().unwrap();
        let titles = title_index(&titles);
        assert!(observed["Nissa Revane"].contains("named Nissa's Chosen"));
        for (name, text) in observed {
            let short = expected[name.as_str()];
            assert_eq!(attested_short_name(&name, &text, &titles), short, "{name}");
            assert_eq!(aliases.get(&name), short, "{name}");
        }
    }

    #[test]
    fn embedded_fragments_are_not_short_reference_attestations() {
        // Negative probes exercise token boundaries, not new Oracle examples.
        let titles = BTreeSet::from(["Dina, Soul Steeper".to_owned(), "Nissa's Chosen".to_owned()]);
        let titles = title_index(&titles);
        for text in ["Dinosaur", "xDina", "Dina-like", "Dina, Soul Steeper"] {
            assert_eq!(
                attested_short_name("Dina, Soul Steeper", text, &titles),
                None
            );
        }
        // Nissa Revane refers to this full card title, not to herself as Nissa.
        assert_eq!(
            attested_short_name("Nissa Revane", "Nissa's Chosen", &titles),
            None
        );
    }
}
