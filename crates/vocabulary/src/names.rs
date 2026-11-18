//! Card names [CR#201.1] and the Card Scope of a face's text
//! [CR#201.5,201.5c].

use std::collections::BTreeSet;

use sylex_lexicon::Binding;
use sylex_lexicon::CardScope;
use sylex_lexicon::Category;
use sylex_lexicon::Lexeme;
use sylex_lexicon::LexicalReading;
use sylex_lexicon::Lexicon;
use sylex_lexicon::NameForm;
use sylex_lexicon::SourceKind;
pub use sylex_lexicon::card_name_id;

use crate::CARD_NAMES;
use crate::SourceError;
use crate::build::source;

/// The Card Scope of the text of a face named `name` with the type line
/// `type_line`: the [`stated_scope`] of that name, a legendary permanent's
/// or not as the type line says ([`crate::legendary_permanent`]). Nothing
/// else of the card is in it, and nothing of its rules text decides it.
///
/// # Errors
/// A type line [`crate::parse_type_line`] cannot read.
pub fn face_scope(
    name: &str,
    type_line: &str,
    lexicon: &Lexicon,
) -> Result<CardScope, SourceError> {
    let (supertypes, types) = crate::parse_type_line(name, type_line)?;
    Ok(stated_scope(
        name,
        crate::legendary_permanent(&supertypes, &types),
        lexicon,
    ))
}

/// The Card Scope of a text read as if on a card named `name`, which need
/// be no card of `lexicon`: its own name in its Full form [CR#201.5], and
/// in its Shortened form [CR#201.5c] too when the card is a legendary
/// permanent and its name has one ([`shortened_name`]).
#[must_use]
pub fn stated_scope(name: &str, legendary_permanent: bool, lexicon: &Lexicon) -> CardScope {
    let candidate = legendary_permanent
        .then(|| sylex_lexicon::shortened_spelling(name))
        .flatten();
    let mut scope = CardScope::full_names([name]);
    if shortened_name(name, candidate.as_deref(), lexicon).is_some() {
        scope.insert(name, NameForm::Shortened);
    }
    scope
}

/// The Shortened Name of the card name `name`, given the spelling its type
/// line and name give it (`candidate`).
///
/// The spelling is dropped where the rest of the vocabulary in scope on that
/// card already spells it (*Sliver* in *Sliver Queen*): a Catalog item such
/// as a token name or an Introduced Name always, any other word, a counter's
/// kind name among them, unless the spelling is the explicit name before a
/// comma or a planeswalker type, which is the name of the character the card
/// is (*Jace* in *Jace, the Mind Sculptor*; *Loot* in *Loot, the
/// Pathfinder*). Another card's name is no such vocabulary: only a position
/// that opts in admits it, so it competes with the Shortened Name nowhere
/// else (*Nicol Bolas* on *Nicol Bolas, the Deceiver*).
#[must_use]
pub fn shortened_name(name: &str, candidate: Option<&str>, lexicon: &Lexicon) -> Option<String> {
    let candidate = candidate?;
    let explicit = name
        .split_once(',')
        .is_some_and(|(head, _)| head.trim() == candidate);
    (candidate != name && !spelled_by_other_vocabulary(candidate, name, explicit, lexicon))
        .then(|| candidate.to_owned())
}

/// Whether other vocabulary in scope on the card named `card` spells all of
/// `candidate`.
fn spelled_by_other_vocabulary(
    candidate: &str,
    card: &str,
    explicit: bool,
    lexicon: &Lexicon,
) -> bool {
    let scope = CardScope::full_names([card]);
    let analyzed = lexicon.analyze_source_scoped(candidate, None, &scope);
    let whole = analyzed.tokens.len();
    analyzed.matches.iter().any(|found| {
        let LexicalReading::Word(value) = &found.reading else {
            return false;
        };
        let lexeme = &lexicon.lexemes()[&value.lexeme];
        found.start == 0
            && found.end == whole
            && !lexeme.card_name
            && (lexeme.category == Category::Catalog && !crate::is_counter_kind_name(lexeme)
                || !explicit
                    && lexeme.binding == Binding::Free
                    && !lexeme.properties.classes.contains("PlaneswalkerType"))
    })
}

/// One Lexeme per card name: the name as another card's text writes it
/// [CR#201.5]. On its own card's faces the name is the card's own, which
/// the face's Card Scope holds ([`face_scope`]).
pub(crate) fn card_names(text: &str) -> Vec<Lexeme> {
    let mut seen = BTreeSet::new();
    text.lines()
        .map(str::trim)
        .filter(|name| !name.is_empty() && seen.insert(*name))
        .map(|name| Lexeme::card_name(name, source(SourceKind::Catalog, CARD_NAMES, name)))
        .collect()
}
