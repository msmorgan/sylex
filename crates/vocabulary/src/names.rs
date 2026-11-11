//! Card names [CR#201.1] and their Shortened Names [CR#201.5c].

use std::collections::BTreeSet;

use sylex_lexicon::Binding;
use sylex_lexicon::Category;
use sylex_lexicon::Lexeme;
use sylex_lexicon::LexemeId;
use sylex_lexicon::LexicalReading;
use sylex_lexicon::Lexicon;
use sylex_lexicon::SourceKind;

use crate::AttestedShortenings;
use crate::CARD_NAMES;
use crate::build::exact;
use crate::build::source;

/// The Shortened Name of a full card name: the shortened spelling its own
/// text uses for it [CR#201.5c] (`attested`). A card whose text never
/// shortens its name has none (*Feast of Flesh* is never *Feast*).
///
/// The spelling is dropped where the rest of the vocabulary in scope on that
/// card already spells it (*Sliver* in *Sliver Queen*): a Catalog item such
/// as a token name or an Introduced Name always, any other word unless the
/// spelling is the explicit name before a comma or a planeswalker type,
/// which is the name of the character the card is (*Jace* in *Jace, the Mind
/// Sculptor*).
#[must_use]
pub fn shortened_name(name: &str, attested: Option<&str>, vocabulary: &Lexicon) -> Option<String> {
    let candidate = attested?.trim();
    let explicit = name
        .split_once(',')
        .is_some_and(|(head, _)| head.trim() == candidate);
    (!candidate.is_empty()
        && candidate != name
        && !spelled_by_other_vocabulary(candidate, card_name_id(name), explicit, vocabulary))
    .then(|| candidate.to_owned())
}

/// Whether other vocabulary in scope on `card` spells all of `candidate`.
fn spelled_by_other_vocabulary(
    candidate: &str,
    card: LexemeId,
    explicit: bool,
    vocabulary: &Lexicon,
) -> bool {
    let scope = BTreeSet::from([card]);
    let analyzed = vocabulary.analyze_source_scoped(candidate, None, &scope);
    let whole = analyzed.tokens.len();
    analyzed.matches.iter().any(|found| {
        let LexicalReading::Word(value) = &found.reading else {
            return false;
        };
        let lexeme = &vocabulary.lexemes()[&value.lexeme];
        found.start == 0
            && found.end == whole
            && (lexeme.category == Category::Catalog
                || !explicit
                    && lexeme.binding == Binding::Free
                    && !lexeme.properties.classes.contains("PlaneswalkerType"))
    })
}

/// The Lexeme id of a card name.
#[must_use]
pub fn card_name_id(name: &str) -> LexemeId {
    LexemeId::new(&format!("name:{name}"))
}

/// One Lexeme per card name, each with its Shortened Name in its own card's
/// scope only. The full name is spelled everywhere: on its own card's faces
/// it is that card's own name, and elsewhere another card's [CR#201.5].
pub(crate) fn card_names(
    text: &str,
    attested: &AttestedShortenings,
    vocabulary: &Lexicon,
) -> Vec<Lexeme> {
    let mut seen = BTreeSet::new();
    text.lines()
        .map(str::trim)
        .filter(|name| !name.is_empty() && seen.insert(*name))
        .map(|name| {
            let mut lexeme = exact(
                card_name_id(name).as_str(),
                name,
                Category::Catalog,
                source(SourceKind::Catalog, CARD_NAMES, name),
            );
            lexeme.card_name = true;
            if let Some(short) = shortened_name(name, attested.get(name), vocabulary) {
                lexeme.forms[0].surfaces = Some(vec![name.to_owned(), short.clone()]);
                lexeme.card_scoped_surfaces = [short].into();
            }
            lexeme
        })
        .collect()
}
