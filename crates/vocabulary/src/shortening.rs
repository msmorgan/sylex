//! Which faces have a Shortened Name [CR#201.5c]: a question of the face's
//! type line. How the name is spelled is `sylex_lexicon::shortened_spelling`.

use sylex_core::Supertype;
use sylex_core::Type;
use sylex_core::TypeDef;

use crate::SourceError;

/// The supertypes and card types a type line prints before its dash.
///
/// # Errors
/// A word before the dash that is neither.
pub fn parse_type_line(
    face: &str,
    type_line: &str,
) -> Result<(Vec<Supertype>, Vec<TypeDef>), SourceError> {
    let head = type_line
        .split_once('—')
        .map_or(type_line, |(head, _)| head);
    let mut supertypes = Vec::new();
    let mut types = Vec::new();
    for word in head.split_whitespace() {
        if let Some(supertype) = Supertype::from_name(word) {
            supertypes.push(supertype);
        } else if let Some(card_type) = Type::from_name(word) {
            types.push(card_type.def());
        } else {
            return Err(SourceError::Invalid(format!(
                "face {face} has unknown word `{word}` in type line `{type_line}`"
            )));
        }
    }
    Ok((supertypes, types))
}

/// Whether these supertypes and card types are a legendary permanent's:
/// the faces whose names Oracle text shortens
/// (`docs/oracle-style-guide.md` §5: "An unambiguous shortened form may
/// replace a long legendary name").
#[must_use]
pub fn legendary_permanent(supertypes: &[Supertype], types: &[TypeDef]) -> bool {
    supertypes.contains(&Supertype::Legendary)
        && types.iter().any(|card_type| card_type.permanent_type)
}

/// The Shortened Name [CR#201.5c] of a face with this name, supertypes and
/// card types: the spelling `sylex_lexicon::shortened_spelling` gives the
/// name, for a [`legendary_permanent`]. None for any other face, and for a
/// name with no shortened spelling. Whether the face's own text uses it is
/// no condition.
#[must_use]
pub fn short_name(name: &str, supertypes: &[Supertype], types: &[TypeDef]) -> Option<String> {
    legendary_permanent(supertypes, types)
        .then(|| sylex_lexicon::shortened_spelling(name))
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_a_legendary_permanent_has_a_shortened_name() {
        let creature = [Type::Creature.def()];
        let legendary = [Supertype::Legendary];
        assert_eq!(
            short_name("Nissa Revane", &legendary, &creature).as_deref(),
            Some("Nissa")
        );
        assert_eq!(short_name("Nissa Revane", &[], &creature), None);
        assert_eq!(
            short_name("Nissa Revane", &legendary, &[Type::Sorcery.def()]),
            None
        );
        // Abuelo, Ancestral Echo's text never shortens its name.
        assert_eq!(
            short_name("Abuelo, Ancestral Echo", &legendary, &creature).as_deref(),
            Some("Abuelo")
        );
        assert_eq!(short_name("Progenitus", &legendary, &creature), None);
    }

    #[test]
    fn a_type_line_gives_its_supertypes_and_card_types() {
        let (supertypes, types) =
            parse_type_line("Karn, Silver Golem", "Legendary Artifact Creature — Golem").unwrap();
        assert_eq!(
            short_name("Karn, Silver Golem", &supertypes, &types).as_deref(),
            Some("Karn")
        );
        let (supertypes, types) =
            parse_type_line("Nissa's Chosen", "Creature — Elf Warrior").unwrap();
        assert_eq!(short_name("Nissa's Chosen", &supertypes, &types), None);
        assert!(parse_type_line("Bad", "Legendary Elf — X").is_err());
    }
}
