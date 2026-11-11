//! Adjacency Agreement: each allomorph is a form selected by a property of
//! the word next to it, never a free choice.

use sylex_lexicon::Category;
use sylex_lexicon::Countability;
use sylex_lexicon::FeatureBundle;
use sylex_lexicon::FormDeclaration;
use sylex_lexicon::GenitiveAlternant;
use sylex_lexicon::Hyphenation;
use sylex_lexicon::Lexeme;
use sylex_lexicon::LexicalReading;
use sylex_lexicon::LexicalValue;
use sylex_lexicon::Lexicon;
use sylex_lexicon::NameForm;
use sylex_lexicon::Onset;
use sylex_lexicon::Source;
use sylex_lexicon::SourceKind;
use sylex_lexicon::SurfaceCase;
use sylex_lexicon::WordForm;

fn source(id: &str) -> Source {
    Source {
        kind: SourceKind::Core,
        path: "agreement-fixture".into(),
        owner: id.into(),
    }
}

fn allomorphs(
    id: &str,
    category: Category,
    forms: impl IntoIterator<Item = (FeatureBundle, &'static str)>,
) -> Lexeme {
    let mut lexeme = Lexeme::invariant(id, id, category, source(id));
    lexeme.forms = forms
        .into_iter()
        .map(|(features, surface)| FormDeclaration {
            form: WordForm::Invariant,
            features,
            surface: Some(surface.into()),
        })
        .collect();
    lexeme
}

fn lexicon() -> Lexicon {
    let article = allomorphs(
        "a",
        Category::Determinative,
        [Onset::Consonant, Onset::Vowel]
            .into_iter()
            .zip(["a", "an"])
            .map(|(onset, surface)| {
                (
                    FeatureBundle {
                        following_onset: Some(onset),
                        ..FeatureBundle::default()
                    },
                    surface,
                )
            }),
    );
    let mut non = allomorphs(
        "non",
        Category::Affix,
        [Hyphenation::Solid, Hyphenation::Hyphenated]
            .into_iter()
            .zip(["non", "non-"])
            .map(|(hyphenation, surface)| {
                (
                    FeatureBundle {
                        hyphenation: Some(hyphenation),
                        ..FeatureBundle::default()
                    },
                    surface,
                )
            }),
    );
    non.binding = sylex_lexicon::Binding::Prefix;
    let mut genitive = allomorphs(
        "genitive",
        Category::Clitic,
        [GenitiveAlternant::ApostropheS, GenitiveAlternant::Bare]
            .into_iter()
            .zip(["'s", "'"])
            .map(|(genitive, surface)| {
                (
                    FeatureBundle {
                        genitive: Some(genitive),
                        ..FeatureBundle::default()
                    },
                    surface,
                )
            }),
    );
    genitive.binding = sylex_lexicon::Binding::Suffix;
    let count_noun = |id: &str| Lexeme::noun(id, id, vec![Countability::Count], source(id));
    let mut vampire = count_noun("Vampire");
    vampire.properties.prefix_hyphenation = Some(Hyphenation::Hyphenated);
    // The zero plural (*four Eldrazi Spawn*).
    let mut spawn = count_noun("Spawn");
    for slot in &mut spawn.forms {
        slot.surface = Some("Spawn".into());
    }
    // A noun spelled alike in both numbers with final *s* (*Thalakos*), an
    // irregular plural (*Elves*), and a singular ending in *s* whose plural
    // differs (*Octopus*, *Octopuses*).
    let mut thalakos = count_noun("Thalakos");
    for slot in &mut thalakos.forms {
        slot.surface = Some("Thalakos".into());
    }
    let octopus = count_noun("Octopus");
    // A card name ending in *s* (*Swarm of Rats's*).
    let rats = Lexeme::card_name(
        "Swarm of Rats",
        "Swarm of Rats",
        None,
        source("Swarm of Rats"),
    );
    let mut elf = count_noun("Elf");
    for slot in &mut elf.forms {
        if slot.form == WordForm::Plural {
            slot.surface = Some("Elves".into());
        }
    }
    Lexicon::new([
        thalakos,
        octopus,
        rats,
        elf,
        article,
        non,
        genitive,
        count_noun("artifact"),
        count_noun("creature"),
        count_noun("land"),
        count_noun("player"),
        vampire,
        spawn,
    ])
    .unwrap()
}

fn value(id: &str, form: WordForm, features: FeatureBundle) -> LexicalReading {
    LexicalReading::Word(LexicalValue {
        lexeme: id.into(),
        form,
        features,
        capitalization: SurfaceCase::Declared,
    })
}

fn noun(id: &str, form: WordForm) -> LexicalReading {
    value(
        id,
        form,
        FeatureBundle {
            number: Some(if form == WordForm::Plural {
                sylex_lexicon::Number::Plural
            } else {
                sylex_lexicon::Number::Singular
            }),
            ..FeatureBundle::default()
        },
    )
}

fn agree(lexicon: &Lexicon, before: &LexicalReading, after: &LexicalReading) -> bool {
    lexicon
        .edges(before)
        .unwrap()
        .right
        .agrees(lexicon.edges(after).unwrap().left)
}

#[test]
fn each_allomorph_realizes_one_spelling_its_neighbor_selects() {
    let lexicon = lexicon();
    let article = |onset| {
        value(
            "a",
            WordForm::Invariant,
            FeatureBundle {
                following_onset: Some(onset),
                ..FeatureBundle::default()
            },
        )
    };
    let (a, an) = (article(Onset::Consonant), article(Onset::Vowel));
    assert_eq!(lexicon.realize(&a).unwrap(), "a");
    assert_eq!(lexicon.realize(&an).unwrap(), "an");
    let creature = noun("creature", WordForm::Singular);
    let artifact = noun("artifact", WordForm::Singular);
    assert!(agree(&lexicon, &a, &creature) && agree(&lexicon, &an, &artifact));
    assert!(!agree(&lexicon, &a, &artifact) && !agree(&lexicon, &an, &creature));

    let prefix = |hyphenation| {
        value(
            "non",
            WordForm::Invariant,
            FeatureBundle {
                hyphenation: Some(hyphenation),
                ..FeatureBundle::default()
            },
        )
    };
    let (solid, hyphenated) = (prefix(Hyphenation::Solid), prefix(Hyphenation::Hyphenated));
    assert_eq!(lexicon.realize(&hyphenated).unwrap(), "non-");
    let land = noun("land", WordForm::Singular);
    let vampire = noun("Vampire", WordForm::Singular);
    assert!(agree(&lexicon, &solid, &land) && agree(&lexicon, &hyphenated, &vampire));
    assert!(!agree(&lexicon, &hyphenated, &land) && !agree(&lexicon, &solid, &vampire));

    let genitive = |alternant| {
        value(
            "genitive",
            WordForm::Invariant,
            FeatureBundle {
                genitive: Some(alternant),
                ..FeatureBundle::default()
            },
        )
    };
    let (suffixed, bare) = (
        genitive(GenitiveAlternant::ApostropheS),
        genitive(GenitiveAlternant::Bare),
    );
    assert_eq!(lexicon.realize(&bare).unwrap(), "'");
    let player = noun("player", WordForm::Singular);
    let players = noun("player", WordForm::Plural);
    // The zero plural ends in no *s*, so it takes the *'s* genitive.
    let spawn = noun("Spawn", WordForm::Plural);
    assert!(agree(&lexicon, &player, &suffixed) && agree(&lexicon, &players, &bare));
    assert!(agree(&lexicon, &spawn, &suffixed) && !agree(&lexicon, &spawn, &bare));
    assert!(!agree(&lexicon, &player, &bare) && !agree(&lexicon, &players, &suffixed));
    // Every Plural ending in *s* takes the bare genitive, an irregular one
    // included, and so does the singular of a noun spelled alike in both
    // numbers with final *s* (CGEL, Ch. 18 §4.2 [35i], p. 1596).
    let thalakos = noun("Thalakos", WordForm::Plural);
    let elves = noun("Elf", WordForm::Plural);
    let one_thalakos = noun("Thalakos", WordForm::Singular);
    assert!(agree(&lexicon, &thalakos, &bare) && !agree(&lexicon, &thalakos, &suffixed));
    assert!(agree(&lexicon, &one_thalakos, &bare) && !agree(&lexicon, &one_thalakos, &suffixed));
    assert!(agree(&lexicon, &elves, &bare) && !agree(&lexicon, &elves, &suffixed));
    // A singular ending in *s* whose plural differs takes *'s*, and so does a
    // name ending in *s*, where CGEL's bare genitive is only a variant [35ii]
    // and Oracle writes *'s* (*Swarm of Rats's*).
    let octopus = noun("Octopus", WordForm::Singular);
    let rats = value(
        "Swarm of Rats",
        WordForm::Invariant,
        FeatureBundle {
            name: Some(NameForm::Full),
            ..FeatureBundle::default()
        },
    );
    assert!(agree(&lexicon, &octopus, &suffixed) && !agree(&lexicon, &octopus, &bare));
    assert!(agree(&lexicon, &rats, &suffixed) && !agree(&lexicon, &rats, &bare));
}

/// Only a Category that hosts a prefix has a prefix hyphenation, so *non*
/// agrees with no word of another (*non-* before the article *a* is no
/// word), and no other Lexeme declares one.
#[test]
fn only_a_prefix_host_has_a_prefix_hyphenation() {
    let lexicon = lexicon();
    let article = value(
        "a",
        WordForm::Invariant,
        FeatureBundle {
            following_onset: Some(Onset::Consonant),
            ..FeatureBundle::default()
        },
    );
    assert_eq!(
        lexicon.edges(&article).unwrap().left.prefix_hyphenation,
        None
    );
    for hyphenation in [Hyphenation::Solid, Hyphenation::Hyphenated] {
        let prefix = value(
            "non",
            WordForm::Invariant,
            FeatureBundle {
                hyphenation: Some(hyphenation),
                ..FeatureBundle::default()
            },
        );
        assert!(!agree(&lexicon, &prefix, &article));
    }
    let mut the = Lexeme::invariant("the", "the", Category::Determinative, source("the"));
    the.properties.prefix_hyphenation = Some(Hyphenation::Hyphenated);
    assert!(Lexicon::new([the]).is_err());
    // A host is solid by default, so only *Hyphenated* is declared.
    let mut land = Lexeme::noun("land", "land", vec![Countability::Count], source("land"));
    land.properties.prefix_hyphenation = Some(Hyphenation::Solid);
    assert!(Lexicon::new([land]).is_err());
}

/// Each dimension belongs to one Category's invariant form, and an agreement
/// dimension's paradigm has a form for each of its values.
#[test]
fn a_dimension_stands_only_on_its_owner_with_every_value() {
    let onset = |onset| FeatureBundle {
        following_onset: Some(onset),
        ..FeatureBundle::default()
    };
    let article = |category| {
        allomorphs(
            "a",
            category,
            [(onset(Onset::Consonant), "a"), (onset(Onset::Vowel), "an")],
        )
    };
    assert!(Lexicon::new([article(Category::Determinative)]).is_ok());
    assert!(Lexicon::new([article(Category::Adverb)]).is_err());
    // *a* without *an*: the context could fix a value it has no form for.
    let only_a = allomorphs(
        "a",
        Category::Determinative,
        [(onset(Onset::Consonant), "a")],
    );
    assert!(Lexicon::new([only_a]).is_err());
    let genitive = |alternant| FeatureBundle {
        genitive: Some(alternant),
        ..FeatureBundle::default()
    };
    let only_bare = allomorphs(
        "genitive",
        Category::Clitic,
        [(genitive(GenitiveAlternant::Bare), "'")],
    );
    assert!(Lexicon::new([only_bare]).is_err());
    let named = allomorphs(
        "Thalakos",
        Category::Noun,
        [(
            FeatureBundle {
                name: Some(NameForm::Full),
                ..FeatureBundle::default()
            },
            "Thalakos",
        )],
    );
    assert!(Lexicon::new([named]).is_err());
}
