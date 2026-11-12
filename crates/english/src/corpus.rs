//! The supported Oracle corpus and its Corpus Units.
//!
//! The supported corpus is every Vintage-playable (legal or restricted) Oracle
//! record outside Scryfall `set_type: funny`. Each Card Face record
//! yields one Type Line unit and one unit per paragraph line of its rules text
//! after balanced reminder text is removed (`docs/decisions/lexicon.md` §1;
//! a line break inside a quotation does not end a line); a face's count is a
//! sum over its units. The split is deterministic, so a text has one
//! segmentation into units.
//!
//! A face's text is read with its recorded Editorial Corrections applied
//! ([`crate::EDITORIAL_CORRECTIONS`]): where the Oracle text breaks a
//! convention its printed card keeps, the census reads the corrected text and
//! the grammar is not relaxed for the slip.

use std::collections::BTreeSet;
use std::io::BufRead;

use sylex_lexicon::CardScope;
use sylex_upstream::oracle_text::strip_reminder_text;
use sylex_upstream::scryfall::OracleCard;
use sylex_upstream::scryfall::OracleCardReader;
use sylex_vocabulary::card_name_id;

/// One Card Face record of the supported corpus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Face {
    /// The Oracle identity and face discriminator (`ORACLE_ID#face:N`).
    pub identity: String,
    /// The face's displayed name.
    pub name: String,
    /// The whole card's name.
    pub card: String,
    pub type_line: Option<String>,
    /// Oracle rules text, reminder text included, with the face's Editorial
    /// Corrections applied.
    pub oracle_text: Option<String>,
}

impl Face {
    /// The Card Scope the face's text is read in: its [`own_names`].
    #[must_use]
    pub fn scope(&self) -> CardScope {
        own_names(&self.name)
    }

    /// The face's units as (kind, index, text): its trimmed type line, then
    /// its rules-text lines ([`split_lines`]).
    fn units(&self) -> impl Iterator<Item = (UnitKind, usize, String)> {
        let type_line = self
            .type_line
            .iter()
            .map(|line| (UnitKind::TypeLine, 0, line.trim().to_owned()));
        let lines = self
            .oracle_text
            .iter()
            .flat_map(|text| split_lines(text).into_iter().enumerate())
            .map(|(line, text)| (UnitKind::Line, line, text));
        type_line.chain(lines)
    }
}

/// One Editorial Correction: `corrected` read in place of `oracle`, which
/// occurs exactly once in the Oracle text of the face named `card`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EditorialCorrection {
    pub card: &'static str,
    pub oracle: &'static str,
    pub corrected: &'static str,
    /// The printed card, or the grammar, that shows the text's error.
    pub evidence: &'static str,
}

impl EditorialCorrection {
    /// The record as one line, for reports.
    #[must_use]
    pub fn describe(&self) -> String {
        format!("{}: {:?} -> {:?}", self.card, self.oracle, self.corrected)
    }
}

/// The tracked Editorial Corrections: blank-line-separated records of
/// `card:`, `oracle:`, `corrected:` and `evidence:` lines, with `#` comments.
///
/// # Panics
/// On a malformed record, which `tests/census.rs` rejects first.
#[must_use]
pub fn editorial_corrections() -> Vec<EditorialCorrection> {
    let mut records = Vec::new();
    let mut fields: Vec<(&'static str, &'static str)> = Vec::new();
    let mut flush = |fields: &mut Vec<(&'static str, &'static str)>| {
        if fields.is_empty() {
            return;
        }
        let field = |key: &str| {
            let values: Vec<_> = fields.iter().filter(|(k, _)| *k == key).collect();
            match values.as_slice() {
                [(_, value)] if !value.is_empty() => *value,
                _ => panic!("editorial correction {fields:?} needs one non-empty `{key}:`"),
            }
        };
        assert_eq!(
            fields.len(),
            4,
            "editorial correction {fields:?} has unknown fields"
        );
        records.push(EditorialCorrection {
            card: field("card"),
            oracle: field("oracle"),
            corrected: field("corrected"),
            evidence: field("evidence"),
        });
        fields.clear();
    };
    for line in crate::EDITORIAL_CORRECTIONS.lines() {
        if line.starts_with('#') {
            continue;
        }
        if line.trim().is_empty() {
            flush(&mut fields);
            continue;
        }
        let (key, value) = line
            .split_once(": ")
            .unwrap_or_else(|| panic!("editorial correction line `{line}` is not `KEY: VALUE`"));
        fields.push((key, value.trim_end()));
    }
    flush(&mut fields);
    records
}

/// `text` with every correction for `name` applied, marking each applied one
/// in `applied`. A correction applies only where its Oracle text occurs once.
fn correct(
    name: &str,
    text: &str,
    corrections: &[EditorialCorrection],
    applied: &mut [bool],
) -> String {
    let mut text = text.to_owned();
    for (correction, applied) in corrections.iter().zip(applied) {
        if correction.card == name && text.matches(correction.oracle).count() == 1 {
            text = text.replacen(correction.oracle, correction.corrected, 1);
            *applied = true;
        }
    }
    text
}

/// What a Corpus Unit is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, serde::Serialize)]
pub enum UnitKind {
    TypeLine,
    /// One paragraph line of rules text.
    Line,
}

/// One Corpus Unit: the smallest span the census enumerates, validates and
/// counts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    /// Index of its face in [`Corpus::faces`].
    pub face: usize,
    pub kind: UnitKind,
    /// Position among the face's rules-text units; 0 for the type line.
    pub index: usize,
    pub text: String,
}

/// The supported corpus, split into units.
#[derive(Debug, Clone, Default)]
pub struct Corpus {
    pub faces: Vec<Face>,
    pub units: Vec<Unit>,
    /// Editorial Corrections that matched no face of the stream read: the
    /// Oracle text they correct has changed or gone.
    pub stale_corrections: Vec<EditorialCorrection>,
    /// Deferral lines (`NAME | TEXT`) that name no unit of any
    /// face of the stream read, kept or not: the unit they name has changed
    /// or gone.
    pub unmatched_deferrals: Vec<String>,
}

/// Why the corpus could not be read.
#[derive(Debug, thiserror::Error)]
pub enum CorpusError {
    #[error(transparent)]
    Snapshot(#[from] &'static sylex_upstream::scryfall::OracleSnapshotError),
    #[error(transparent)]
    Read(#[from] sylex_upstream::scryfall::OracleCardReadError),
}

/// The Card Scope of a face named `name`: its own name in every form
/// [CR#201.5c]. Nothing else of the card is in it, so the scope of a text is
/// computed from the name of the face it is on.
///
/// Each face of a double-faced card has its own name [CR#712.6], and two faces
/// that shorten to one spelling (*Ral*) name one object, not two, so a face's
/// text shortens its own name. The whole card's name (*A // B*) is no card-name
/// Lexeme, and a meld result's name is another card's on both cards of its pair
/// (`docs/decisions/lexicon.md` §6).
#[must_use]
pub fn own_names(name: &str) -> CardScope {
    CardScope::own_names([card_name_id(name)])
}

/// The rules-text units of a face: its paragraph lines after balanced
/// reminder text is removed (a keyword line's semicolon separators becoming
/// commas with it), trimmed, without empty lines. A line break
/// inside a quotation that closes later in the text does not end a line.
#[must_use]
pub fn split_lines(oracle_text: &str) -> Vec<String> {
    let text = strip_reminder_text(oracle_text);
    let lines: Vec<&str> = text.lines().collect();
    let quotes = |line: &str| line.matches('"').count();
    let mut units = Vec::new();
    let mut start = 0;
    while start < lines.len() {
        let mut end = start + 1;
        let mut open = quotes(lines[start]) % 2 == 1;
        while open && let Some(close) = lines[end..].iter().position(|l| quotes(l) > 0) {
            end += close + 1;
            open = lines[start..end].iter().map(|l| quotes(l)).sum::<usize>() % 2 == 1;
        }
        let unit = lines[start..end].join("\n");
        let unit = unit.trim();
        if !unit.is_empty() {
            units.push(unit.to_owned());
        }
        start = end;
    }
    units
}

impl Corpus {
    /// Reads the supported corpus from an Oracle Cards JSON-lines stream,
    /// keeping the faces for which `keep(face)` holds.
    ///
    /// # Errors
    /// An unreadable or invalid record.
    pub fn read(reader: impl BufRead, keep: impl Fn(&Face) -> bool) -> Result<Self, CorpusError> {
        Self::read_deferring(reader, keep, crate::census::deferrals())
    }

    /// As [`Corpus::read`], matching `deferrals` (`NAME | TEXT` pairs) in
    /// place of `deferred.txt`'s.
    fn read_deferring(
        reader: impl BufRead,
        keep: impl Fn(&Face) -> bool,
        deferrals: BTreeSet<(&str, &str)>,
    ) -> Result<Self, CorpusError> {
        let mut builder = Builder::new(deferrals);
        for card in OracleCardReader::new(reader) {
            builder.card(&card?, &keep);
        }
        Ok(builder.finish())
    }

    /// The supported corpus of the pinned snapshot at
    /// `data/scryfall/oracle-cards.jsonl`, from the records the process
    /// holds (`sylex_upstream::scryfall::oracle_cards`).
    ///
    /// # Errors
    /// A missing or invalid snapshot.
    pub fn load(keep: impl Fn(&Face) -> bool) -> Result<Self, CorpusError> {
        let mut builder = Builder::new(crate::census::deferrals());
        for card in sylex_upstream::scryfall::oracle_cards()? {
            builder.card(card, &keep);
        }
        Ok(builder.finish())
    }

    /// Adds a face and its units.
    pub fn push(&mut self, face: Face) {
        let index = self.faces.len();
        self.units
            .extend(face.units().map(|(kind, line, text)| Unit {
                face: index,
                kind,
                index: line,
                text,
            }));
        self.faces.push(face);
    }
}

/// Builds a [`Corpus`] card by card, tracking which Editorial Corrections
/// applied and which deferrals matched.
struct Builder<'d> {
    corpus: Corpus,
    corrections: Vec<EditorialCorrection>,
    applied: Vec<bool>,
    unmatched: BTreeSet<(&'d str, &'d str)>,
}

impl<'d> Builder<'d> {
    fn new(deferrals: BTreeSet<(&'d str, &'d str)>) -> Self {
        let corrections = editorial_corrections();
        Self {
            corpus: Corpus::default(),
            applied: vec![false; corrections.len()],
            corrections,
            unmatched: deferrals,
        }
    }

    /// Adds the faces of `card` that `keep` holds for, if it is supported.
    fn card(&mut self, card: &OracleCard, keep: impl Fn(&Face) -> bool) {
        if !card.supported() {
            return;
        }
        for unit in card.oracle_units() {
            let face = Face {
                identity: unit.identity().to_string(),
                name: unit.display_name().to_owned(),
                card: unit.group_name().to_owned(),
                type_line: unit.type_line().map(str::to_owned),
                oracle_text: unit.oracle_text().map(|text| {
                    correct(
                        unit.display_name(),
                        text,
                        &self.corrections,
                        &mut self.applied,
                    )
                }),
            };
            if !self.unmatched.is_empty() {
                let texts: Vec<String> = face.units().map(|(_, _, text)| text).collect();
                self.unmatched
                    .retain(|&(name, text)| name != face.name || !texts.iter().any(|t| t == text));
            }
            if keep(&face) {
                self.corpus.push(face);
            }
        }
    }

    fn finish(self) -> Corpus {
        let mut corpus = self.corpus;
        corpus.stale_corrections = self
            .corrections
            .into_iter()
            .zip(self.applied)
            .filter_map(|(correction, applied)| (!applied).then_some(correction))
            .collect();
        corpus.unmatched_deferrals = self
            .unmatched
            .into_iter()
            .map(|(name, text)| format!("{name} | {text}"))
            .collect();
        corpus
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_drop_reminder_text_and_blank_lines() {
        assert_eq!(
            split_lines(
                "Flying (This creature can't be blocked.)\n(Reminder only.)\n{T}: Add {G}."
            ),
            ["Flying", "{T}: Add {G}."]
        );
    }

    #[test]
    fn keyword_semicolons_become_commas_with_the_reminder_text() {
        // Shu General's Oracle text.
        assert_eq!(
            split_lines(
                "Vigilance; horsemanship (This creature can't be blocked except by \
                 creatures with horsemanship.)"
            ),
            ["Vigilance, horsemanship"]
        );
    }

    #[test]
    fn a_line_break_inside_a_quotation_does_not_end_a_line() {
        assert_eq!(
            split_lines("Creatures you control have \"{T}: Add {G}.\n{T}: Untap this.\"\nFlying"),
            [
                "Creatures you control have \"{T}: Add {G}.\n{T}: Untap this.\"",
                "Flying"
            ]
        );
        // A quotation that never closes does not swallow the lines after it.
        assert_eq!(
            split_lines("Say \"hello\nFlying"),
            ["Say \"hello", "Flying"]
        );
        assert_eq!(
            split_lines("\"A\" and \"B\nC\"\nD"),
            ["\"A\" and \"B\nC\"", "D"]
        );
    }

    #[test]
    fn a_recorded_editorial_correction_is_read_in_place_of_the_oracle_text() {
        // Agency Outfitter's Oracle text drops the serial comma its printed
        // card has; the correction is recorded in `corrections.txt`.
        let input = r#"{"object":"card","id":"p","oracle_id":"o","name":"Agency Outfitter","layout":"normal","legalities":{"vintage":"legal"},"type_line":"Creature — Sphinx Detective","oracle_text":"Search your graveyard, hand and/or library for a card."}"#;
        let corpus = Corpus::read(std::io::Cursor::new(input), |_| true).unwrap();
        assert_eq!(
            corpus.faces[0].oracle_text.as_deref(),
            Some("Search your graveyard, hand, and/or library for a card.")
        );
        assert!(
            corpus
                .stale_corrections
                .iter()
                .all(|c| c.card != "Agency Outfitter")
        );
    }

    #[test]
    fn a_correction_whose_oracle_text_is_gone_is_stale() {
        let input = r#"{"object":"card","id":"p","oracle_id":"o","name":"Agency Outfitter","layout":"normal","legalities":{"vintage":"legal"},"type_line":"Creature — Sphinx Detective","oracle_text":"Search your graveyard, hand, and/or library for a card."}"#;
        let corpus = Corpus::read(std::io::Cursor::new(input), |_| true).unwrap();
        assert_eq!(
            corpus.faces[0].oracle_text.as_deref(),
            Some("Search your graveyard, hand, and/or library for a card.")
        );
        assert!(
            corpus
                .stale_corrections
                .iter()
                .any(|c| c.card == "Agency Outfitter")
        );
    }

    #[test]
    fn a_deferral_matches_a_unit_of_any_face_read_kept_or_not() {
        // Lord of the Pit's real text names the first deferral; the second
        // names a unit of no face of the stream.
        let line = "Lord of the Pit | Flying, trample";
        let deferrals = || {
            BTreeSet::from([
                ("Lord of the Pit", "Flying, trample"),
                ("Lord of the Pit", "Fear"),
            ])
        };
        let input = r#"{"object":"card","id":"p","oracle_id":"o","name":"Lord of the Pit","layout":"normal","legalities":{"vintage":"legal"},"type_line":"Creature — Demon","oracle_text":"Flying, trample\nAt the beginning of your upkeep, sacrifice a creature other than this creature. If you can't, this creature deals 7 damage to you."}"#;
        let corpus =
            Corpus::read_deferring(std::io::Cursor::new(input), |_| false, deferrals()).unwrap();
        assert_eq!(corpus.unmatched_deferrals, ["Lord of the Pit | Fear"]);
        let corpus =
            Corpus::read_deferring(std::io::Cursor::new(""), |_| true, deferrals()).unwrap();
        assert!(corpus.unmatched_deferrals.iter().any(|l| l == line));
    }

    #[test]
    fn a_face_has_its_own_name_in_scope_and_not_its_other_face_s() {
        // Both faces shorten to *Ral*; each face's text names only itself.
        let input = r#"{"object":"card","id":"p","oracle_id":"o","name":"Ral, Monsoon Mage // Ral, Leyline Prodigy","layout":"transform","legalities":{"vintage":"legal"},"card_faces":[{"name":"Ral, Monsoon Mage","type_line":"Creature — Human Wizard","oracle_text":"Transform Ral."},{"name":"Ral, Leyline Prodigy","type_line":"Legendary Planeswalker — Ral","oracle_text":"Ral deals 2 damage to any target."}]}"#;
        let corpus = Corpus::read(std::io::Cursor::new(input), |_| true).unwrap();
        let scopes: Vec<_> = corpus.faces.iter().map(Face::scope).collect();
        assert_eq!(
            scopes,
            [
                CardScope::own_names([card_name_id("Ral, Monsoon Mage")]),
                CardScope::own_names([card_name_id("Ral, Leyline Prodigy")]),
            ]
        );
        // The whole card's name is the name of no face.
        let card = card_name_id("Ral, Monsoon Mage // Ral, Leyline Prodigy");
        assert!(scopes.iter().all(|scope| !scope.holds_card(card)));
    }

    #[test]
    fn a_meld_card_s_scope_holds_its_own_name_and_not_the_meld_result_s() {
        let parts = r#""all_parts":[{"object":"related_card","id":"r","component":"meld_result","name":"Brisela, Voice of Nightmares","type_line":"Legendary Creature — Eldrazi Angel"},{"object":"related_card","id":"g","component":"meld_part","name":"Gisela, the Broken Blade","type_line":"Legendary Creature — Angel Horror"},{"object":"related_card","id":"b","component":"meld_part","name":"Bruna, the Fading Light","type_line":"Legendary Creature — Angel Horror"}]"#;
        let input = [
            format!(
                r#"{{"object":"card","id":"g","oracle_id":"g","name":"Gisela, the Broken Blade","layout":"meld","legalities":{{"vintage":"legal"}},{parts}}}"#
            ),
            format!(
                r#"{{"object":"card","id":"b","oracle_id":"b","name":"Bruna, the Fading Light","layout":"meld","legalities":{{"vintage":"legal"}},{parts}}}"#
            ),
            r#"{"object":"card","id":"l","oracle_id":"l","name":"Lightning Bolt","layout":"normal","legalities":{"vintage":"legal"}}"#.to_owned(),
        ]
        .join("\n");
        let corpus = Corpus::read(std::io::Cursor::new(input), |_| true).unwrap();
        let scopes: Vec<_> = corpus.faces.iter().map(Face::scope).collect();
        assert_eq!(
            scopes,
            [
                CardScope::own_names([card_name_id("Gisela, the Broken Blade")]),
                CardScope::own_names([card_name_id("Bruna, the Fading Light")]),
                CardScope::own_names([card_name_id("Lightning Bolt")]),
            ]
        );
        // The meld result's name is another card's on both cards of the pair.
        let result = card_name_id("Brisela, Voice of Nightmares");
        assert!(scopes.iter().all(|scope| !scope.holds_card(result)));
    }

    #[test]
    fn a_face_yields_its_type_line_and_each_line() {
        let mut corpus = Corpus::default();
        corpus.push(Face {
            identity: "x#card".to_owned(),
            name: "Seedborn Muse".to_owned(),
            card: "Seedborn Muse".to_owned(),
            type_line: Some("Creature — Spirit".to_owned()),
            oracle_text: Some(
                "Untap all permanents you control during each other player's untap step."
                    .to_owned(),
            ),
        });
        let units: Vec<_> = corpus
            .units
            .iter()
            .map(|u| (u.kind, u.text.as_str()))
            .collect();
        assert_eq!(
            units,
            [
                (UnitKind::TypeLine, "Creature — Spirit"),
                (
                    UnitKind::Line,
                    "Untap all permanents you control during each other player's untap step."
                ),
            ]
        );
    }
}
