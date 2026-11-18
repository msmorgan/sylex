//! Introduced Names on the real lines that introduce them
//! (`docs/design/lexicon.md` §6): a name is read from its text where the
//! text introduces it, named again only in a Reading that introduces it,
//! and never a card's name.

use std::sync::LazyLock;

use sylex_lexicon::LexicalReading;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::Reading;
use sylex_vocabulary::LexicalEnvironment;

static ENV: LazyLock<(LexicalEnvironment, CompiledGrammar)> = LazyLock::new(|| {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    (env, grammar)
});

/// A face's name and type line, and one line of its text.
type Line = (&'static str, &'static str, &'static str);

/// The Readings of a line, on its face.
fn readings((face, type_line, text): Line) -> Vec<Reading> {
    let (env, grammar) = &*ENV;
    let scope = sylex_vocabulary::face_scope(face, type_line, &env.lexicon).unwrap();
    let analyzed = env.lexicon.analyze_source_scoped(text, None, &scope);
    let parse = grammar.parse_analyzed(&analyzed);
    assert_eq!(parse.vocabulary_gaps, [], "{face}");
    let found: Vec<_> = parse.readings().collect();
    assert!(!found.is_empty(), "{face}: no Reading");
    for reading in &found {
        assert_eq!(
            grammar.admit(reading, &env.lexicon, &scope),
            Ok(()),
            "{face}"
        );
    }
    found
}

/// Each name leaf of a Reading in order, with the Production over it: an
/// Introduced Name, or another card's name by its Lexeme id.
fn names(reading: &Reading, found: &mut Vec<(String, String)>) {
    let Reading::Node(node) = reading else { return };
    for child in &node.children {
        match child {
            Reading::Leaf(LexicalReading::IntroducedName { name }) => {
                found.push((node.production.to_string(), name.clone()));
            }
            Reading::Leaf(LexicalReading::Word(value))
                if value.lexeme.as_str().starts_with("name:") =>
            {
                found.push((node.production.to_string(), value.lexeme.to_string()));
            }
            child => names(child, found),
        }
    }
}

/// The name leaves every Reading of a line has: the same in each.
fn named(line: Line) -> Vec<(String, String)> {
    let mut all: Vec<Vec<(String, String)>> = readings(line)
        .iter()
        .map(|reading| {
            let mut found = Vec::new();
            names(reading, &mut found);
            found
        })
        .collect();
    all.dedup();
    assert_eq!(
        all.len(),
        1,
        "{}: Readings differ in their names: {all:?}",
        line.0
    );
    all.remove(0)
}

fn pairs(expected: &[(&str, &str)]) -> Vec<(String, String)> {
    expected
        .iter()
        .map(|(production, name)| ((*production).to_owned(), (*name).to_owned()))
        .collect()
}

const RINOA: Line = (
    "Rinoa Heartilly",
    "Legendary Creature — Human Rebel Warlock",
    "When Rinoa Heartilly enters, create Angelo, a legendary 1/1 green and white Dog creature token.",
);

#[test]
fn each_construction_introduces_the_name_written_there() {
    // `create NAME, a ... token`: the description introduces the Object.
    assert_eq!(named(RINOA), pairs(&[("NamedTokenName", "Angelo")]));
    // `... token named NAME`.
    assert_eq!(
        named((
            "Peer Review",
            "Sorcery",
            "Create a 2/2 colorless Wizard Soldier creature token named Cadet. Surveil 1.",
        )),
        pairs(&[("NameAfterNamed", "Cadet")])
    );
    // `becomes ... named NAME` and `She's ... named NAME`: a permanent the
    // text changes.
    assert_eq!(
        named((
            "The Curse of Fenric",
            "Enchantment — Saga",
            "II — Target nontoken creature becomes a 6/6 legendary Horror creature named Fenric and loses all abilities.",
        )),
        pairs(&[("NameAfterNamed", "Fenric")])
    );
    assert_eq!(
        named((
            "Princess Yue",
            "Legendary Creature — Human Noble Ally",
            "When Princess Yue dies, if she was a nonland creature, return this card to the battlefield tapped under your control. She's a land named Moon. She gains \"{T}: Add {C}.\"",
        )),
        pairs(&[("NameAfterNamed", "Moon")])
    );
    // `except its name is NAME`.
    assert_eq!(
        named((
            "Mishra, Eminent One",
            "Legendary Creature — Human Artificer",
            "At the beginning of combat on your turn, create a token that's a copy of target noncreature artifact you control, except its name is Mishra's Warform and it's a 4/4 Construct artifact creature in addition to its other types. It gains haste until end of turn. Sacrifice it at the beginning of the next end step.",
        )),
        pairs(&[("NameSpecification", "Mishra's Warform")])
    );
    // `create a NAME token`, NAME a card's name [CR#111.11].
    assert_eq!(
        named((
            "Disa the Restless",
            "Legendary Creature — Human Scout",
            "Whenever one or more creatures you control deal combat damage to a player, create a Tarmogoyf token.",
        )),
        pairs(&[("CardNameBeforeToken", "name:Tarmogoyf")])
    );
    assert_eq!(
        named((
            "Mutable Explorer",
            "Creature — Shapeshifter",
            "When this creature enters, create a tapped Mutavault token.",
        )),
        pairs(&[("CardNameBeforeToken", "name:Mutavault")])
    );
}

#[test]
fn a_name_is_named_again_where_its_text_introduces_it() {
    // After *named* on another line of the face: the Complement of *named*
    // introduces the name wherever it stands.
    assert_eq!(
        named((
            "The Curse of Fenric",
            "Enchantment — Saga",
            "III — Target Mutant fights another target creature named Fenric.",
        )),
        pairs(&[("NameAfterNamed", "Fenric")])
    );
    // On a face that introduces the name nowhere else.
    assert_eq!(
        named((
            "Hand of Vecna",
            "Legendary Artifact — Equipment",
            "At the beginning of combat on your turn, equipped creature or a creature you control named Vecna gets +X/+X until end of turn, where X is the number of cards in your hand.",
        )),
        pairs(&[("NameAfterNamed", "Vecna")])
    );
    // In the token's own quoted ability, which names the token [CR#201.5].
    assert_eq!(
        named((
            "The Coming of Galactus",
            "Enchantment — Saga",
            "IV — Create Galactus, a legendary 16/16 black Elder Alien creature token with flying, trample, and \"Whenever Galactus attacks, destroy target land.\"",
        )),
        pairs(&[
            ("NamedTokenName", "Galactus"),
            ("RepeatedIntroducedName", "Galactus")
        ])
    );
    // As a possessor there.
    assert_eq!(
        named((
            "Bonny Pall, Clearcutter",
            "Legendary Creature — Giant Scout",
            "When Bonny Pall enters, create Beau, a legendary blue Ox creature token with \"Beau's power and toughness are each equal to the number of lands you control.\"",
        )),
        pairs(&[
            ("NamedTokenName", "Beau"),
            ("RepeatedIntroducedName", "Beau")
        ])
    );
    // In a later sentence.
    assert_eq!(
        named((
            "Kari Zev, Skyship Raider",
            "Legendary Creature — Human Pirate",
            "Whenever Kari Zev attacks, create Ragavan, a legendary 2/1 red Monkey creature token. Ragavan enters tapped and attacking. Exile that token at end of combat.",
        )),
        pairs(&[
            ("NamedTokenName", "Ragavan"),
            ("RepeatedIntroducedName", "Ragavan")
        ])
    );
    // The Object of *create* offers the name anew, after *named* introduced
    // it: one Reading, with no name named again.
    let jiang = (
        "Jiang Yanggu",
        "Legendary Planeswalker — Yanggu",
        "−1: If you don't control a creature named Mowu, create Mowu, a legendary 3/3 green Dog creature token.",
    );
    assert_eq!(readings(jiang).len(), 1);
    assert_eq!(
        named(jiang),
        pairs(&[("NameAfterNamed", "Mowu"), ("NamedTokenName", "Mowu")])
    );
}

#[test]
fn a_name_with_a_comma_is_one_name() {
    assert_eq!(
        named((
            "Tolsimir, Friend to Wolves",
            "Legendary Creature — Elf Scout",
            "When Tolsimir enters, create Voja, Friend to Elves, a legendary 3/3 green and white Wolf creature token.",
        )),
        pairs(&[("NamedTokenName", "Voja, Friend to Elves")])
    );
    assert_eq!(
        named((
            "Tenth District Hero",
            "Creature — Human",
            "{2}{W}, Collect evidence 4: If this creature is a Detective, it becomes a legendary creature named Mileva, the Stalwart, it has base power and toughness 5/5, and it gains \"Other creatures you control have indestructible.\"",
        )),
        pairs(&[("NameAfterNamed", "Mileva, the Stalwart")])
    );
    assert_eq!(
        named((
            "Nissa, Sage Animist",
            "Legendary Planeswalker — Nissa",
            "−2: Create Ashaya, the Awoken World, a legendary 4/4 green Elemental creature token.",
        )),
        pairs(&[("NamedTokenName", "Ashaya, the Awoken World")])
    );
}

#[test]
fn a_card_s_name_is_never_read_as_an_introduced_name() {
    // Each card name after *named* is another card's name or the face's
    // own, whole; only *Kaldra* is introduced.
    let kaldra = named((
        "Helm of Kaldra",
        "Legendary Artifact — Equipment",
        "{1}: If you control Equipment named Helm of Kaldra, Sword of Kaldra, and Shield of Kaldra, create Kaldra, a legendary 4/4 colorless Avatar creature token. Attach those Equipment to it.",
    ));
    assert_eq!(
        kaldra,
        pairs(&[
            ("OtherCardNameComplement", "name:Sword of Kaldra"),
            ("OtherCardNameComplement", "name:Shield of Kaldra"),
            ("NamedTokenName", "Kaldra"),
        ])
    );
    for (line, card_names) in [
        (
            (
                "Urza, Lord Protector",
                "Legendary Creature — Human Artificer",
                "{7}: If you both own and control Urza, Lord Protector and an artifact named The Mightstone and Weakstone, exile them, then meld them into Urza, Planeswalker. Activate only as a sorcery.",
            ),
            &[
                "name:The Mightstone and Weakstone",
                "name:Urza, Planeswalker",
            ][..],
        ),
        (
            (
                "Skoa, Embermage",
                "Legendary Creature — Goblin Wizard",
                "Grandeur — Discard another card named Skoa, Embermage, Sacrifice two Mountains: Skoa deals 4 damage to any target.",
            ),
            &[][..],
        ),
        (
            (
                "Vanille, Cheerful l'Cie",
                "Legendary Creature — Human Cleric",
                "At the beginning of your first main phase, if you both own and control Vanille and a creature named Fang, Fearless l'Cie, you may pay {3}{B}{G}. If you do, exile them, then meld them into Ragnarok, Divine Deliverance.",
            ),
            &[
                "name:Fang, Fearless l'Cie",
                "name:Ragnarok, Divine Deliverance",
            ][..],
        ),
    ] {
        let found: Vec<String> = named(line).into_iter().map(|(_, name)| name).collect();
        assert_eq!(found, card_names, "{}", line.0);
    }
    // *Blood* is the predefined token there, never the card named Blood.
    let blood = (
        "Blood Fountain",
        "Artifact",
        "When this artifact enters, create a Blood token.",
    );
    assert_eq!(readings(blood).len(), 1);
    assert_eq!(named(blood), []);
}

#[test]
fn a_name_stands_only_where_its_text_introduces_it() {
    let (env, grammar) = &*ENV;
    let scope = sylex_vocabulary::stated_scope("Rinoa Heartilly", true, &env.lexicon);
    let count = |text: &str| {
        let analyzed = env.lexicon.analyze_source_scoped(text, None, &scope);
        let parse = grammar.parse_analyzed(&analyzed);
        (parse.readings().count(), parse.vocabulary_gaps.len())
    };
    // The Object of *create* without its description names nothing, and the
    // title run is a word no vocabulary holds.
    assert_eq!(count("Create Angelo."), (0, 1));
    assert_eq!(count("Angelo attacks."), (0, 1));
    assert_eq!(
        count("Sacrifice Angelo, a legendary 1/1 green and white Dog creature token."),
        (0, 1)
    );
    // A name another name's introduction does not introduce.
    assert_eq!(
        count("Create Angelo, a legendary 1/1 green and white Dog creature token. Boo attacks."),
        (0, 1)
    );
    // A Reading that names the name again is refused apart from the text
    // that introduces it, and on a face named as it is.
    let whole = readings((
        "Kari Zev, Skyship Raider",
        "Legendary Creature — Human Pirate",
        "Whenever Kari Zev attacks, create Ragavan, a legendary 2/1 red Monkey creature token. Ragavan enters tapped and attacking.",
    ));
    let kari = sylex_vocabulary::stated_scope("Kari Zev, Skyship Raider", true, &env.lexicon);
    let mut repeated = None;
    let mut stack = vec![&whole[0]];
    while let Some(reading) = stack.pop() {
        let Reading::Node(node) = reading else { continue };
        if node.category.as_str() == "Sentence"
            && grammar.realize(reading, &env.lexicon).unwrap()
                == "Ragavan enters tapped and attacking."
        {
            repeated = Some(reading);
        }
        stack.extend(&node.children);
    }
    let repeated = repeated.expect("the sentence that names Ragavan again");
    assert_eq!(
        grammar.admit_category(repeated, &env.lexicon, "Sentence", &kari),
        Err(sylex_syntax::AdmissionError::NotIntroduced)
    );
    let ragavan = sylex_vocabulary::stated_scope("Ragavan", false, &env.lexicon);
    assert!(grammar.admit(&whole[0], &env.lexicon, &ragavan).is_err());
}
