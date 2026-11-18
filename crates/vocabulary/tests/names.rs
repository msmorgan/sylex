//! Card names [CR#201.1] and their Shortened Names [CR#201.5c], which are in
//! scope only on their own card.

use sylex_lexicon::CardScope;
use sylex_lexicon::LexicalReading;
use sylex_lexicon::NameForm;
use sylex_lexicon::card_name_id;
use sylex_vocabulary::LexicalEnvironment;
use sylex_vocabulary::Sources;
use sylex_vocabulary::face_scope;
use sylex_vocabulary::shortened_name;
use sylex_vocabulary::stated_scope;

/// A face's name in its Shortened form is in its Card Scope by its name and
/// type line alone, whether or not its text ever shortens the name.
#[test]
fn a_legendary_permanent_has_a_shortened_name_whatever_its_text_says() {
    let lexicon = LexicalEnvironment::load().unwrap().lexicon;
    for (name, type_line, expected) in [
        (
            "Gandalf the Grey",
            "Legendary Creature — Avatar Wizard",
            true,
        ),
        (
            "Daxos of Meletis",
            "Legendary Creature — Human Soldier",
            true,
        ),
        ("Edgar Markov", "Legendary Creature — Vampire Knight", true),
        // The explicit name before a comma is the character's own name.
        (
            "Wolverine, Best There Is",
            "Legendary Creature — Mutant Berserker Hero",
            true,
        ),
        ("Karn Liberated", "Legendary Planeswalker — Karn", true),
        (
            "King Darien XLVIII",
            "Legendary Creature — Human Soldier",
            true,
        ),
        (
            "Halana and Alena, Partners",
            "Legendary Creature — Human Ranger",
            true,
        ),
        // Their text never shortens their names.
        (
            "Abuelo, Ancestral Echo",
            "Legendary Creature — Spirit",
            true,
        ),
        (
            "Gisela, Blade of Goldnight",
            "Legendary Creature — Angel",
            true,
        ),
        // *Sliver* is a creature type on every card.
        ("Sliver Queen", "Legendary Creature — Sliver", false),
        // A possessive name is a description.
        ("Gaea's Cradle", "Legendary Land", false),
        ("Marit Lage's Slumber", "Legendary Snow Enchantment", false),
        // Only a legendary permanent's name is shortened.
        ("Yawgmoth's Vile Offering", "Legendary Sorcery", false),
        ("Urabrask's Forge", "Artifact", false),
        ("Feast of Flesh", "Sorcery", false),
        ("Counterspell", "Instant", false),
    ] {
        let scope = face_scope(name, type_line, &lexicon).unwrap();
        assert!(scope.holds(name, NameForm::Full), "{name}");
        assert_eq!(scope.holds(name, NameForm::Shortened), expected, "{name}");
    }
    assert!(face_scope("Counterspell", "Instant Elf", &lexicon).is_err());
}

/// A text may be read as if on a card of any name, a legendary permanent's
/// or not as stated.
#[test]
fn a_stated_card_has_its_name_in_scope_and_shortens_it_when_legendary() {
    let lexicon = LexicalEnvironment::load().unwrap().lexicon;
    for name in ["Abuelo, Ancestral Echo", "Abuelo the Brave"] {
        assert_eq!(
            stated_scope(name, true, &lexicon),
            CardScope::every_form([name])
        );
        assert_eq!(
            stated_scope(name, false, &lexicon),
            CardScope::full_names([name])
        );
    }
    assert_eq!(
        stated_scope("Sliver Queen", true, &lexicon),
        CardScope::full_names(["Sliver Queen"])
    );
}

#[test]
fn a_shortened_name_never_spells_vocabulary_in_scope_on_its_card() {
    let lexicon = LexicalEnvironment::load().unwrap().lexicon;
    let short = |name: &str, candidate: &str| shortened_name(name, Some(candidate), &lexicon);
    // A planeswalker type, or the name before a comma, is the character's
    // own name even where other words spell it.
    assert_eq!(
        short("Jace, the Mind Sculptor", "Jace").as_deref(),
        Some("Jace")
    );
    assert_eq!(
        short("Wolverine, Best There Is", "Wolverine").as_deref(),
        Some("Wolverine")
    );
    assert_eq!(short("Goblin Guide", "Goblin"), None);
    // An Introduced Name is other vocabulary on the cards that introduce it,
    // and only there.
    assert_eq!(short("Hornet Cannon", "Hornet"), None);
    assert_eq!(short("Wirefly Hive", "Wirefly"), None);
    assert_eq!(short("Hornet Nest", "Hornet").as_deref(), Some("Hornet"));
    // A token name is other vocabulary even before a comma.
    assert_eq!(short("Walker, Night Stalker", "Walker"), None);
    // Another card's name is no vocabulary a Shortened Name competes with:
    // only a position that opts in admits it.
    assert_eq!(
        short("Nicol Bolas, the Deceiver", "Nicol Bolas").as_deref(),
        Some("Nicol Bolas")
    );
}

#[test]
fn a_name_is_its_card_s_own_in_its_scope_and_another_card_s_elsewhere() {
    let lexicon = LexicalEnvironment::load().unwrap().lexicon;
    let name = "Dina, Soul Steeper";
    let own = |form| LexicalReading::OwnName {
        name: name.to_owned(),
        form,
    };
    let other = card_name_id(name);
    let found = |text: &str, scope: &CardScope| -> Vec<LexicalReading> {
        lexicon
            .analyze_source_scoped(text, None, scope)
            .matches
            .into_iter()
            .map(|found| found.reading)
            .filter(|reading| match reading {
                LexicalReading::Word(value) => value.lexeme == other,
                LexicalReading::OwnName { .. } => true,
                LexicalReading::Numeral { .. } | LexicalReading::FlavorWord { .. } => false,
            })
            .collect()
    };
    let scope = stated_scope(name, true, &lexicon);
    assert_eq!(found("Dina", &scope), [own(NameForm::Shortened)]);
    assert_eq!(found("Dina", &CardScope::default()), []);
    // The full name is spelled everywhere: off its own card the grammar
    // reads it as another card's name, only after *named*, *Partner with* or
    // *from among*.
    assert!(found(name, &scope).contains(&own(NameForm::Full)));
    let elsewhere = found(name, &CardScope::default());
    assert!(
        matches!(elsewhere.as_slice(), [LexicalReading::Word(value)] if value.lexeme == other),
        "{elsewhere:?}"
    );
}

/// Whether `text` analyzes, whole, as `lexeme` with `scope`'s cards in scope.
fn spells(environment: &LexicalEnvironment, text: &str, lexeme: &str, scope: &[&str]) -> bool {
    let scope = CardScope::full_names(scope.iter().copied());
    let analyzed = environment
        .lexicon
        .analyze_source_scoped(text, None, &scope);
    let whole = analyzed.tokens.len();
    analyzed.matches.iter().any(|found| {
        found.start == 0
            && found.end == whole
            && matches!(&found.reading, LexicalReading::Word(value) if value.lexeme == lexeme)
    })
}

#[test]
fn an_introduced_name_is_in_scope_only_on_the_cards_that_use_it() {
    let environment = LexicalEnvironment::load().unwrap();
    for (name, card) in [
        ("Marit Lage", "Dark Depths"),
        ("Marit Lage", "Marit Lage's Slumber"),
        ("Icingdeath, Frost Tongue", "Icingdeath, Frost Tyrant"),
        ("Tuktuk the Returned", "Tuktuk the Explorer"),
        ("Moon", "Princess Yue"),
    ] {
        let id = format!("introduced_name:{name}");
        assert!(spells(&environment, name, &id, &[card]), "{name} on {card}");
        assert!(!spells(&environment, name, &id, &[]), "{name} elsewhere");
        assert!(
            !spells(&environment, name, &id, &["Vampire Hexmage"]),
            "{name} on another card"
        );
    }
    // Its spelling is exact.
    assert!(!spells(
        &environment,
        "moon",
        "introduced_name:Moon",
        &["Princess Yue"]
    ));
}

#[test]
fn every_introducing_card_admits_its_introduced_names() {
    let environment = LexicalEnvironment::load().unwrap();
    let sources = Sources::load().unwrap();
    for introduced in &sources.introduced_names {
        let id = format!("introduced_name:{}", introduced.name);
        for card in &introduced.cards {
            assert!(
                spells(&environment, &introduced.name, &id, &[card]),
                "{} on {card}",
                introduced.name
            );
        }
    }
}
