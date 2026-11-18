//! Vote Options [CR#701.38a,701.38b] are read from a face's text
//! (`docs/design/lexicon.md` §2), with no list of the words a card prints.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use sylex_english::census::Verdict;
use sylex_english::corpus::Corpus;
use sylex_lexicon::CardScope;
use sylex_lexicon::LexicalReading;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::Reading;
use sylex_vocabulary::LexicalEnvironment;

/// Each card whose vote offers words, with those words.
const WORD_VOTES: [(&str, [&str; 2]); 26] = [
    ("Bite of the Black Rose", ["psychosis", "sickness"]),
    ("Capital Punishment", ["death", "taxes"]),
    ("Coercive Portal", ["carnage", "homage"]),
    ("Elrond of the White Council", ["aid", "fellowship"]),
    ("Emissary Green", ["profit", "security"]),
    ("Expropriate", ["money", "time"]),
    ("Fateful Tempest", ["past", "present"]),
    ("Galadriel, Elven-Queen", ["dominion", "guidance"]),
    ("Lieutenants of the Guard", ["numbers", "strength"]),
    ("Magister of Worth", ["condemnation", "grace"]),
    ("Messenger Jays", ["feather", "quill"]),
    ("Orchard Elemental", ["harvest", "sprout"]),
    ("Path of the Animist", ["chaos", "planeswalk"]),
    ("Path of the Enigma", ["chaos", "planeswalk"]),
    ("Path of the Ghosthunter", ["chaos", "planeswalk"]),
    ("Path of the Pyromancer", ["chaos", "planeswalk"]),
    ("Path of the Schemer", ["chaos", "planeswalk"]),
    ("Plea for Power", ["knowledge", "time"]),
    ("Sail into the West", ["embark", "return"]),
    ("Selvala's Stampede", ["free", "wild"]),
    ("Split Decision", ["denial", "duplication"]),
    ("Tivit, Seller of Secrets", ["bribery", "evidence"]),
    (
        "Travel Through Caradhras",
        ["Mines of Moria", "Redhorn Pass"],
    ),
    ("Trial of a Time Lord", ["guilty", "innocent"]),
    ("Truth or Consequences", ["consequences", "truth"]),
    ("Tyrant's Choice", ["death", "torture"]),
];

/// Cards that vote, or say how votes are cast, and offer no words: their
/// options are objects, players or colors [CR#701.38b].
const OTHER_VOTES: [&str; 14] = [
    "Ballot Broker",
    "Brago's Representative",
    "Council Guardian",
    "Council's Judgment",
    "Custodi Squire",
    "Círdan the Shipwright",
    "Erestor of the Council",
    "Grudge Keeper",
    "Illusion of Choice",
    "Mob Verdict",
    "Model of Unity",
    "The Valeyard",
    "Trap the Trespassers",
    "Vault 11: Voter's Dilemma",
];

const BITE: &str = "Will of the council — Starting with you, each player votes for sickness or \
                    psychosis. If sickness gets more votes, creatures your opponents control get \
                    -2/-2 until end of turn. If psychosis gets more votes or the vote is tied, \
                    each opponent discards two cards.";
const PATH: &str = "Will of the Planeswalkers — Starting with you, each player votes for \
                    planeswalk or chaos. If planeswalk gets more votes, planeswalk. If chaos gets \
                    more votes or the vote is tied, chaos ensues.";

fn vote_options(
    env: &LexicalEnvironment,
    grammar: &CompiledGrammar,
    card: &str,
    lines: &[&str],
) -> Vec<String> {
    let lines: Vec<String> = lines.iter().map(|line| (*line).to_owned()).collect();
    sylex_english::options::vote_options(
        &lines,
        &CardScope::full_names([card]),
        &env.lexicon,
        grammar,
    )
    .into_iter()
    .collect()
}

fn scope(card: &str, options: &[&str]) -> CardScope {
    let mut scope = CardScope::full_names([card]);
    for word in options {
        scope.insert_vote_option(word);
    }
    scope
}

fn readings(
    env: &LexicalEnvironment,
    grammar: &CompiledGrammar,
    text: &str,
    scope: &CardScope,
) -> Vec<Reading> {
    grammar
        .parse_analyzed(&env.lexicon.analyze_source_scoped(text, None, scope))
        .readings()
        .collect()
}

/// The Vote Option leaves of a Reading, in order, each with the Production
/// that holds it.
fn options_of(reading: &Reading, production: &str, found: &mut Vec<(String, String)>) {
    match reading {
        Reading::Leaf(LexicalReading::VoteOption { word }) => {
            found.push((word.clone(), production.to_owned()));
        }
        Reading::Leaf(_) => {}
        Reading::Node(node) => node
            .children
            .iter()
            .for_each(|child| options_of(child, node.production.as_str(), found)),
    }
}

/// Every unit of every card that votes is read, round trip included, with
/// the Vote Options its face's own text gives it: the two words its vote
/// offers on a card that offers words, and none on any other.
#[test]
fn every_voting_card_reads_with_the_vote_options_of_its_own_text() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let expected: BTreeMap<&str, Vec<&str>> = WORD_VOTES
        .iter()
        .map(|(card, words)| (*card, words.to_vec()))
        .chain(OTHER_VOTES.iter().map(|card| (*card, Vec::new())))
        .collect();
    let corpus = Corpus::load(|face| expected.contains_key(face.name.as_str())).unwrap();
    let read: BTreeMap<&str, Vec<String>> = corpus
        .faces
        .iter()
        .map(|face| {
            let scope = face.scope(&env.lexicon, &grammar);
            let options = scope.vote_options().map(str::to_owned).collect();
            (face.name.as_str(), options)
        })
        .collect();
    let expected: BTreeMap<&str, Vec<String>> = expected
        .into_iter()
        .map(|(card, words)| (card, words.into_iter().map(str::to_owned).collect()))
        .collect();
    assert_eq!(read, expected);
    let report = sylex_english::census::run(&corpus, &env, &[]);
    let unread: Vec<String> = report
        .units
        .iter()
        .filter(|unit| unit.verdict != Verdict::Read)
        .map(|unit| format!("{}: {:?}: {}", unit.name, unit.verdict, unit.text))
        .collect();
    assert_eq!(unread, [] as [String; 0]);
}

/// A vote that offers two words makes them Vote Options only where the text
/// then cites one of them [CR#701.38b]: Bite of the Black Rose's offer
/// alone gives none, with either effect it gives both, and the effects
/// without the offer give none.
#[test]
fn an_offer_is_confirmed_by_a_mention_of_one_of_its_words() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let none: [&str; 0] = [];
    let card = "Bite of the Black Rose";
    let offer = "Starting with you, each player votes for sickness or psychosis.";
    let sickness = "If sickness gets more votes, creatures your opponents control get -2/-2 \
                    until end of turn.";
    let psychosis = "If psychosis gets more votes or the vote is tied, each opponent discards \
                     two cards.";
    assert_eq!(
        vote_options(&env, &grammar, card, &[BITE]),
        ["psychosis", "sickness"]
    );
    assert_eq!(vote_options(&env, &grammar, card, &[offer]), none);
    // The mention may stand on another line of the face.
    for effect in [sickness, psychosis] {
        assert_eq!(
            vote_options(&env, &grammar, card, &[offer, effect]),
            ["psychosis", "sickness"]
        );
    }
    assert_eq!(
        vote_options(&env, &grammar, card, &[sickness, psychosis]),
        none
    );
    // Trial of a Time Lord never cites *innocent*: the vote that offers it
    // offers *guilty*, which it does cite.
    assert_eq!(
        vote_options(
            &env,
            &grammar,
            "Trial of a Time Lord",
            &[
                "IV — Starting with you, each player votes for innocent or guilty. If guilty gets \
               more votes, the owner of each card exiled with this Saga puts that card on the \
               bottom of their library."
            ]
        ),
        ["guilty", "innocent"]
    );
}

/// With its Vote Options in scope Bite of the Black Rose has one Reading:
/// the vote offers the two as an Option Choice and each effect cites its
/// word as the Subject of the clause that counts its votes.
#[test]
fn the_offer_and_each_mention_read_their_vote_options() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let held = scope("Bite of the Black Rose", &["sickness", "psychosis"]);
    let found = readings(&env, &grammar, BITE, &held);
    let [reading] = &found[..] else {
        panic!("{} Readings", found.len());
    };
    let mut options = Vec::new();
    options_of(reading, "", &mut options);
    let options: Vec<(&str, &str)> = options
        .iter()
        .map(|(word, production)| (word.as_str(), production.as_str()))
        .collect();
    assert_eq!(
        options,
        [
            ("sickness", "OptionWordChoice"),
            ("psychosis", "OptionWordChoice"),
            ("sickness", "OptionWordMention"),
            ("psychosis", "OptionWordMention"),
        ]
    );
}

/// Admission refuses a Vote Option the scope does not hold, and admits it
/// where the scope does: the parse and admission take the one scope
/// (`docs/design/english.md` §7).
#[test]
fn admission_takes_a_vote_option_only_in_a_scope_that_holds_it() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let card = "Bite of the Black Rose";
    let held = scope(card, &["sickness", "psychosis"]);
    let other = scope(card, &["sickness"]);
    let bare = scope(card, &[]);
    let found = readings(&env, &grammar, BITE, &held);
    let [reading] = &found[..] else {
        panic!("{} Readings", found.len());
    };
    assert_eq!(grammar.admit(reading, &env.lexicon, &held), Ok(()));
    for scope in [&other, &bare] {
        assert!(grammar.admit(reading, &env.lexicon, scope).is_err());
        assert_eq!(readings(&env, &grammar, BITE, scope), []);
    }
}

/// On the five Path of the ... cards the vote offers *chaos*, and the last
/// clause uses the noun (*chaos ensues*): no Reading cites the option
/// there. The one fork left is the Subject of *gets more votes*, the noun
/// or the cited word.
#[test]
fn a_vote_option_is_cited_only_where_its_votes_are_counted() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    let held = scope("Path of the Enigma", &["planeswalk", "chaos"]);
    let found = readings(&env, &grammar, PATH, &held);
    let cited: BTreeSet<Vec<(String, String)>> = found
        .iter()
        .map(|reading| {
            let mut options = Vec::new();
            options_of(reading, "", &mut options);
            options
        })
        .collect();
    let pair = |word: &str, production: &str| (word.to_owned(), production.to_owned());
    let offer = [
        pair("planeswalk", "OptionWordChoice"),
        pair("chaos", "OptionWordChoice"),
        pair("planeswalk", "OptionWordMention"),
    ];
    let mut with_mention = offer.to_vec();
    with_mention.push(pair("chaos", "OptionWordMention"));
    assert_eq!(found.len(), 2);
    assert_eq!(cited, BTreeSet::from([offer.to_vec(), with_mention]));
}
