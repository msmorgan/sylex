//! Provisional: the schemas `.syl` does not declare.
//!
//! Declaration kinds, record sorts and sort aliases have no `.syl`
//! declaration; their only other copy is the Lean workbench (`SylGen`'s
//! `kindSort`, `allowedFields` and `body`, and the `Semantics` structures and
//! `abbrev`s), which step 5 of the semantics-v3 plan deletes. Once it does,
//! these tables are their only copy, so whether they move into `.syl` is open.
//! Everything that is a schema without a `.syl` source is here, in one place,
//! and nowhere else in the validator.

/// What a declaration kind's block holds, besides `lexical` and `spelling`.
pub struct Kind {
    pub word: &'static str,
    /// The sort a declaration of this kind declares its name at; `None` when
    /// the header writes it (`primitive`, `template`).
    pub sort: Option<&'static str>,
    /// The fields its block may write.
    pub fields: &'static [FieldSpec],
    /// A second sort its name is declared at (`syl.md` §3).
    pub secondary: Option<&'static str>,
    /// A catalog declaration (a counter, designation, subtype, card type or
    /// relation) is registered before any body is read, so a reference to
    /// one is no dependency edge.
    pub catalog: bool,
    /// Instead of a `body`, the block may write the fields of the kind's
    /// record sort directly.
    pub record_fields: bool,
    /// The one rule of this kind that no field's sort states.
    pub rule: KindRule,
}

/// A kind's rule beyond its fields' sorts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KindRule {
    None,
    /// `identity` is a nonempty string, on a parameterless `NounPhrase`
    /// template.
    Identity,
    /// `ofAbility` excludes `deed` and `permanent`.
    OfAbility,
    /// An ability word is wrapped by the `abilityWord` declaration at
    /// `Ability`.
    AbilityWord,
    /// Temporary: `kind = spell` is read as text, a word that never resolves.
    /// `syl-subtype-card-types` deletes this exception.
    SpellKind,
}

/// One field of a declaration kind's block.
pub struct FieldSpec {
    pub name: &'static str,
    /// `None`: the declaration's own sort.
    pub sort: Option<&'static str>,
    pub required: bool,
}

const fn field(name: &'static str, sort: &'static str) -> FieldSpec {
    FieldSpec {
        name,
        sort: Some(sort),
        required: false,
    }
}

const fn required(name: &'static str, sort: &'static str) -> FieldSpec {
    FieldSpec {
        name,
        sort: Some(sort),
        required: true,
    }
}

/// A catalog kind's conferrals.
const CONFERS: FieldSpec = field("confers", "List(Conferral)");
const RULES: FieldSpec = field("rules", "List(Conferral)");

const fn kind(word: &'static str, sort: &'static str, fields: &'static [FieldSpec]) -> Kind {
    Kind {
        word,
        sort: Some(sort),
        fields,
        secondary: None,
        catalog: false,
        record_fields: false,
        rule: KindRule::None,
    }
}

const fn catalog(word: &'static str, sort: &'static str, fields: &'static [FieldSpec]) -> Kind {
    Kind {
        catalog: true,
        ..kind(word, sort, fields)
    }
}

const fn rules_row(word: &'static str, sort: &'static str) -> Kind {
    Kind {
        record_fields: true,
        ..kind(
            word,
            sort,
            &[FieldSpec {
                name: "body",
                sort: None,
                required: false,
            }],
        )
    }
}

/// The kind word of a card, the one nameless declaration.
pub const CARD: &str = "card";

/// Every declaration kind.
pub const KINDS: &[Kind] = &[
    Kind {
        sort: None,
        ..kind("primitive", "", &[])
    },
    Kind {
        sort: None,
        rule: KindRule::Identity,
        ..kind(
            "template",
            "",
            &[
                FieldSpec {
                    name: "body",
                    sort: None,
                    required: true,
                },
                field("identity", "String"),
            ],
        )
    },
    Kind {
        secondary: Some("Deed"),
        rule: KindRule::OfAbility,
        ..kind(
            "action",
            "Action",
            &[
                required("body", "Action"),
                field("deed", "Option(String)"),
                field("permanent", "DeedPermanent"),
                field("ofAbility", "KeywordLabel"),
            ],
        )
    },
    Kind {
        secondary: Some("KeywordLabel"),
        ..kind(
            "ability",
            "Ability",
            &[field("body", "List(Ability)"), field("label", "String")],
        )
    },
    Kind {
        rule: KindRule::AbilityWord,
        ..kind(
            "ability_word",
            "Ability",
            &[required("body", "Ability"), field("label", "String")],
        )
    },
    catalog(
        "counter",
        "CounterKind",
        &[field("holder", "Kind"), CONFERS, RULES],
    ),
    catalog(
        "designation",
        "DesignationLabel",
        &[
            required("scope", "DesignationScope"),
            field("sole", "Bool"),
            CONFERS,
            RULES,
        ],
    ),
    Kind {
        rule: KindRule::SpellKind,
        ..catalog(
            "subtype",
            "Subtype",
            &[
                required("kind", "CardType"),
                field("name", "String"),
                CONFERS,
                RULES,
            ],
        )
    },
    catalog(
        "card_type",
        "CardType",
        &[
            field("name", "String"),
            field("cardClass", "Option(CardClass)"),
            field("stats", "List(Stat)"),
            CONFERS,
            RULES,
        ],
    ),
    catalog("relation", "RelationLabel", &[]),
    rules_row("token", "PredefinedToken"),
    rules_row("sba", "SbaRule"),
    rules_row("conferral", "ConferralRule"),
    kind(CARD, "Card", &[]),
];

/// A record sort, written `{ field = value, ... }`: its fields, each with its
/// sort and whether it may be omitted.
pub struct RecordSort {
    pub sort: &'static str,
    pub fields: &'static [(&'static str, &'static str, bool)],
}

const fn record(
    sort: &'static str,
    fields: &'static [(&'static str, &'static str, bool)],
) -> RecordSort {
    RecordSort { sort, fields }
}

const MANA_COST: &str = "Option(List(ManaSymbol))";

/// Every record sort. `CardType` is a record in the model but has none here:
/// only a `card_type` declaration builds one.
pub const RECORDS: &[RecordSort] = &[
    record("CardFace", &[("characteristics", "Characteristics", false)]),
    record(
        "CharacteristicBundle",
        &[
            ("characteristics", "Characteristics", false),
            ("qualities", "List(WrittenQuality)", true),
        ],
    ),
    record(
        "Characteristics",
        &[
            ("name", "Option(String)", true),
            ("cost", MANA_COST, true),
            ("colors", "List(Color)", true),
            ("supertypes", "List(Supertype)", true),
            ("types", "List(CardType)", true),
            ("subtypes", "List(Subtype)", true),
            ("text", "List(Ability)", true),
            ("power", "Option(Amount)", true),
            ("toughness", "Option(Amount)", true),
            ("loyalty", "Option(Amount)", true),
            ("defense", "Option(Amount)", true),
        ],
    ),
    record(
        "ConferralRule",
        &[
            ("scope", "Predicate", false),
            ("confer", "Conferral", false),
        ],
    ),
    record(
        "DeonticMods",
        &[
            ("timing", "Option(Timing)", true),
            ("asThough", "Option(AsThough)", true),
            ("except", "Option(DeonticExcept)", true),
            ("cause", "Option(Causing)", true),
            ("rider", "Option(Action)", true),
        ],
    ),
    record(
        "DidClause",
        &[
            ("subj", "Subject", false),
            ("action", "Action", false),
            ("within", "Option(LookbackSpan)", false),
            ("ordinal", "Option(EventOrdinal)", true),
        ],
    ),
    record(
        "EventOrdinal",
        &[
            ("ordinal", "Ordinal", false),
            ("per", "Option(Period)", true),
        ],
    ),
    record(
        "FaceDown",
        &[("characteristics", "Option(Characteristics)", false)],
    ),
    record(
        "JoinedHeader",
        &[
            ("event", "GameEvent", false),
            ("alternatives", "List(GameEvent)", false),
            ("while", "Option(Timing)", false),
            ("timing", "Option(Timing)", false),
        ],
    ),
    record(
        "KeywordFamily",
        &[
            ("word", "KeywordLabel", false),
            ("sort", "Option(QualitySort)", true),
        ],
    ),
    record(
        "LevelBand",
        &[
            ("range", "LevelRange", false),
            ("text", "List(Ability)", true),
            ("power", "Option(Amount)", true),
            ("toughness", "Option(Amount)", true),
        ],
    ),
    record(
        "ManaSubstitute",
        &[
            ("action", "Action", false),
            ("pays", "List(PipPaid)", false),
        ],
    ),
    record(
        "Mode",
        &[
            ("cost", "Option(Cost)", true),
            ("weight", "Option(Nat)", true),
            ("action", "Action", false),
        ],
    ),
    record(
        "PredefinedToken",
        &[
            ("name", "String", false),
            ("token", "CharacteristicBundle", false),
        ],
    ),
    record(
        "PrototypeFrame",
        &[
            ("cost", MANA_COST, true),
            ("power", "Option(Amount)", true),
            ("toughness", "Option(Amount)", true),
        ],
    ),
    record(
        "RollRow",
        &[("results", "Quantity", false), ("action", "Action", false)],
    ),
    record(
        "RulesTables",
        &[
            ("sba", "List(SbaRule)", true),
            ("conferral", "List(ConferralRule)", true),
        ],
    ),
    record(
        "SbaRule",
        &[
            ("scope", "Predicate", false),
            ("when", "Condition", false),
            ("then", "Action", false),
        ],
    ),
    record(
        "SharedLineHalf",
        &[
            ("name", "String", false),
            ("cost", MANA_COST, true),
            ("text", "List(Ability)", true),
        ],
    ),
    record(
        "TypeLineChanges",
        &[
            ("supertypes", "Option(List(Supertype))", true),
            ("types", "Option(List(CardType))", true),
            ("subtypes", "Option(List(Subtype))", true),
        ],
    ),
];

/// Sort names written for another sort, which every sort is read as. Whether
/// they are respelled to the sort they stand for or kept is open.
pub const ALIASES: &[(&str, &str)] = &[
    ("FlavorWordLabel", "String"),
    ("VoteLabel", "String"),
    ("ManaCost", "List(ManaSymbol)"),
    ("ProducedRun", "List(ColorOrColorless)"),
];
