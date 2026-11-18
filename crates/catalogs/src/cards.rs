use std::collections::BTreeSet;
use std::io::BufRead;

use sylex_upstream::scryfall::OracleCard;
use sylex_upstream::scryfall::OracleCardReader;
use sylex_upstream::scryfall::OracleUnit;

use crate::CatalogError;

/// The names the Oracle card data supplies.
pub(crate) struct Names {
    /// The face names of the supported cards.
    pub(crate) cards: BTreeSet<String>,
    /// The names of the dungeon cards [CR#309.1], which begin outside the
    /// game [CR#309.2]: the faces of card type Dungeon on token-set records,
    /// the oversized Commander Party aids (*Baldur's Gate Wilderness*) apart.
    pub(crate) dungeons: BTreeSet<String>,
}

pub(crate) fn extract(oracle_cards: impl BufRead) -> Result<Names, CatalogError> {
    let mut names = Names {
        cards: BTreeSet::new(),
        dungeons: BTreeSet::new(),
    };

    for card in OracleCardReader::new(oracle_cards) {
        let card = card?;
        if card.supported() {
            for unit in card.oracle_units() {
                names.cards.insert(face_name(&unit, "card-names")?);
            }
        }
        if is_dungeon_record(&card) {
            for unit in card.oracle_units().filter(is_dungeon) {
                names.dungeons.insert(face_name(&unit, "dungeon-names")?);
            }
        }
    }

    Ok(names)
}

fn face_name(unit: &OracleUnit<'_>, catalog: &str) -> Result<String, CatalogError> {
    let selected = unit.display_name();
    if selected.contains(" // ") {
        return Err(CatalogError::CompositeName {
            catalog: catalog.into(),
            name: selected.into(),
        });
    }
    Ok(selected.to_owned())
}

/// A token-set record of play size: where Scryfall files the dungeon cards.
fn is_dungeon_record(card: &OracleCard) -> bool {
    !card.oversized && card.set_type.as_deref() == Some("token")
}

/// Whether a face has card type Dungeon: a word of its type line before the
/// subtypes.
fn is_dungeon(unit: &OracleUnit<'_>) -> bool {
    unit.type_line().is_some_and(|line| {
        line.split(" — ")
            .next()
            .is_some_and(|types| types.split_whitespace().any(|word| word == "Dungeon"))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_from_funny_sets_are_excluded_even_when_vintage_legal() {
        let records = concat!(
            r#"{"object":"card","id":"normal-p","oracle_id":"normal-o","name":"Normal","layout":"normal","set_type":"expansion","legalities":{"vintage":"legal"}}"#,
            "\n",
            r#"{"object":"card","id":"funny-p","oracle_id":"funny-o","name":"Funny","layout":"normal","set_type":"funny","legalities":{"vintage":"legal"}}"#,
        );
        assert_eq!(
            extract(records.as_bytes()).unwrap().cards,
            BTreeSet::from(["Normal".into()])
        );
    }

    /// The dungeon faces of play-size token-set records, a double-faced
    /// token's dungeon face among them (*Undercity // The Initiative*), and
    /// no oversized aid, no non-dungeon face and no card naming a dungeon.
    #[test]
    fn dungeon_names_are_the_dungeon_faces_of_token_records() {
        let records = concat!(
            r#"{"object":"card","id":"tomb-p","oracle_id":"tomb-o","name":"Tomb of Annihilation","layout":"normal","set_type":"token","oversized":false,"type_line":"Dungeon","legalities":{}}"#,
            "\n",
            r#"{"object":"card","id":"undercity-p","oracle_id":"undercity-o","name":"Undercity // The Initiative","layout":"double_faced_token","set_type":"token","oversized":false,"legalities":{},"card_faces":[{"name":"Undercity","type_line":"Dungeon — Undercity"},{"name":"The Initiative","type_line":"Card"}]}"#,
            "\n",
            r#"{"object":"card","id":"wilderness-p","oracle_id":"wilderness-o","name":"Baldur's Gate Wilderness","layout":"normal","set_type":"token","oversized":true,"type_line":"Dungeon","legalities":{}}"#,
            "\n",
            r#"{"object":"card","id":"skeleton-p","oracle_id":"skeleton-o","name":"Skeleton","layout":"token","set_type":"token","type_line":"Token Creature — Skeleton","legalities":{}}"#,
            "\n",
            r#"{"object":"card","id":"dungeon-map-p","oracle_id":"dungeon-map-o","name":"Dungeon Map","layout":"normal","set_type":"expansion","type_line":"Artifact","legalities":{"vintage":"legal"}}"#,
        );
        let names = extract(records.as_bytes()).unwrap();
        assert_eq!(
            names.dungeons,
            BTreeSet::from(["Tomb of Annihilation".into(), "Undercity".into()])
        );
        assert_eq!(names.cards, BTreeSet::from(["Dungeon Map".into()]));
    }
}
