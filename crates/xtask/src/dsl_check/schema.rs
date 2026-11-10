//! Provisional: the schemas `.syl` does not declare.
//!
//! Declaration kinds, record sorts and the reserved constructors have no
//! `.syl` declaration, so they are provisional Rust tables until step 7's AST
//! states them.
//! Everything that is a schema without a `.syl` source is here, in one place,
//! and nowhere else in the validator. Items marked "step 6" are ones
//! `docs/cleanup/06-syl-iteration.md` plans to reshape or delete.

/// What a declaration kind's block holds, besides `lexical` and `spelling`.
pub struct Kind {
    pub word: &'static str,
    /// How a declaration of this kind is written.
    pub form: Form,
    /// The sort a declaration of this kind declares its name at; `None` when
    /// the header writes it (`primitive`, `template`, `coercion`).
    pub sort: Option<&'static str>,
    /// The fields its block may write.
    pub fields: &'static [FieldSpec],
    /// A second sort its name is declared at (`syl.md` §3).
    pub secondary: Option<Secondary>,
    /// A catalog declaration (a counter, designation, subtype, card type,
    /// supertype, relation or top-level rules row) is registered before any
    /// body is read, so a reference to one is no dependency edge.
    pub catalog: bool,
    /// When the block writes none of `fields`, it writes the fields of the
    /// kind's record sort directly.
    pub record_fields: bool,
}

/// How a declaration kind is written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Form {
    /// A name and a `{ ... }` block.
    Block,
    /// A name, with the block optional.
    OptionalBlock,
    /// A value alone, with no name: a card.
    Value,
    /// A header with no name, and a block: a coercion.
    NamelessBlock,
}

/// A second sort a kind declares its name at.
#[derive(Debug, Clone, Copy)]
pub struct Secondary {
    pub sort: &'static str,
    /// No second declaration when the block writes one of these fields: any
    /// value for `None`, or the bare word given.
    pub unless: &'static [(&'static str, Option<&'static str>)],
    /// Which constructors of `sort` a primitive may not build.
    pub reserves: Reserves,
}

/// The constructors of a sort that no primitive builds.
#[derive(Debug, Clone, Copy)]
pub enum Reserves {
    /// Every one but a label: the constructor a primitive with no parameters
    /// names after itself, capitalized (`primitive firstStrike: KeywordLabel =
    /// FirstStrike`), which declares a Primitive Keyword Ability's label.
    AllButLabels,
    /// Only these; the sort's other constructors have primitives.
    Only(&'static [&'static str]),
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

const fn kind(word: &'static str, sort: &'static str, fields: &'static [FieldSpec]) -> Kind {
    Kind {
        word,
        form: Form::Block,
        sort: Some(sort),
        fields,
        secondary: None,
        catalog: false,
        record_fields: false,
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

/// The kind word of the declaration that binds a word to a constructor.
pub const PRIMITIVE: &str = "primitive";

/// The kind word of a coercion, which declares no word (`syl.md` §3).
pub const COERCION: &str = "coercion";

/// Every declaration kind.
pub const KINDS: &[Kind] = &[
    Kind {
        sort: None,
        form: Form::OptionalBlock,
        ..kind(PRIMITIVE, "", &[])
    },
    Kind {
        sort: None,
        ..kind(
            "template",
            "",
            &[FieldSpec {
                name: "body",
                sort: None,
                required: true,
            }],
        )
    },
    Kind {
        sort: None,
        form: Form::NamelessBlock,
        ..kind(
            COERCION,
            "",
            &[FieldSpec {
                name: "body",
                sort: None,
                required: true,
            }],
        )
    },
    Kind {
        secondary: Some(Secondary {
            sort: "Deed",
            unless: &[("ofAbility", None), ("deed", Some("none"))],
            // Step 6: `.syl` writes Deeds explicitly, so a raw Deed is no
            // longer reserved.
            reserves: Reserves::Only(&["Action"]),
        }),
        ..kind(
            "action",
            "Action",
            &[
                required("body", "Action"),
                // Step 6: explicit Deeds (a deed action kind) replace these
                // three fields.
                field("deed", "Option(String)"),
                field("permanent", "DeedPermanent"),
                field("ofAbility", "KeywordLabel"),
            ],
        )
    },
    Kind {
        secondary: Some(Secondary {
            sort: "KeywordLabel",
            unless: &[],
            reserves: Reserves::AllButLabels,
        }),
        ..kind(
            "ability",
            "Ability",
            &[field("body", "List(Ability)"), field("label", "String")],
        )
    },
    kind(
        "ability_word",
        "Ability",
        &[required("body", "Ability"), field("label", "String")],
    ),
    catalog(
        "counter",
        "CounterKind",
        &[field("holder", "Kind"), CONFERS],
    ),
    catalog(
        "designation",
        "DesignationLabel",
        &[
            required("scope", "DesignationScope"),
            field("sole", "Bool"),
            CONFERS,
        ],
    ),
    catalog(
        "subtype",
        "Subtype",
        &[
            // The card types it is correlated with [CR#205.3c,205.3d].
            required("kind", "List(CardType)"),
            field("name", "String"),
            CONFERS,
        ],
    ),
    catalog(
        "card_type",
        "CardType",
        &[
            field("name", "String"),
            required("permanentType", "Bool"),
            field("stats", "List(Stat)"),
            CONFERS,
        ],
    ),
    catalog(
        "supertype",
        "Supertype",
        &[field("name", "String"), CONFERS],
    ),
    catalog("relation", "RelationLabel", &[]),
    rules_row("token", "DefinedToken"),
    // A top-level rules row is a catalog declaration, so card text can name
    // it as it names a keyword: the "legend rule" [CR#205.4d].
    Kind {
        catalog: true,
        ..rules_row("sba", "SbaRule")
    },
    Kind {
        catalog: true,
        ..rules_row("conferral", "ConferralRule")
    },
    Kind {
        form: Form::Value,
        ..kind("card", "Card", &[])
    },
];

/// The sort constructors, each taking one sort argument.
pub const SORT_CONSTRUCTORS: [&str; 3] = ["Option", "List", "Delta"];

/// Constructors only expansion writes, each at its sort (`None`: at every
/// sort that has it). `syl.md` §4 names the categories (a scope, a mention, a
/// raw Deed, a keyword wrapper); this list is provisional, and step 7's AST
/// replaces it.
// Step 6: explicit Deeds let `.syl` write a raw Deed, reshaping this list.
const EXPANSION_CONSTRUCTORS: &[(Option<&str>, &str)] = &[
    (None, "InCaller"),
    (Some("NounPhrase"), "FirstMention"),
    (Some("NounPhrase"), "LaterMention"),
    (Some("Action"), "OwnScope"),
    (Some("Ability"), "Keyword"),
];

/// The reserved constructor the primitive `name`, with `params` parameters,
/// builds at `sort` by naming `constructor`, as `Sort.Constructor`: one only
/// expansion writes, or any constructor of a catalog kind's sort, or one a
/// second sort reserves.
#[must_use]
pub fn reserved(name: &str, params: usize, sort: &str, constructor: &str) -> Option<String> {
    let expansion = EXPANSION_CONSTRUCTORS
        .iter()
        .any(|(at, word)| *word == constructor && at.is_none_or(|at| at == sort));
    let label = params == 0 && is_capitalized(name, constructor);
    let declared = KINDS.iter().any(|kind| {
        (kind.catalog && kind.sort == Some(sort))
            || kind.secondary.is_some_and(|secondary| {
                secondary.sort == sort
                    && match secondary.reserves {
                        Reserves::AllButLabels => !label,
                        Reserves::Only(names) => names.contains(&constructor),
                    }
            })
    });
    (expansion || declared).then(|| format!("{sort}.{constructor}"))
}

/// Whether `constructor` is `name` with its first letter capitalized.
fn is_capitalized(name: &str, constructor: &str) -> bool {
    let mut chars = name.chars();
    chars.next().is_some_and(|first| {
        let capitalized: String = first.to_uppercase().chain(chars).collect();
        capitalized == constructor
    })
}

/// Temporary: the one file that may declare a primitive building a reserved
/// constructor, relative to the builtin bundle, and the constructor. Only that
/// file may call it. `syl-ring-tempts-you-raw-scope` deletes this exception.
pub const RING_EXCEPTION: (&str, &str) = (
    "definitions/keyword_action/theRingTemptsYou.syl",
    "Action.OwnScope",
);

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
        "DefinedToken",
        &[
            ("characteristics", "Characteristics", false),
            ("back", "Option(Characteristics)", true),
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
        "TriggerHeader",
        &[
            ("event", "GameEvent", false),
            ("while", "Option(Timing)", false),
            ("timing", "Option(Timing)", false),
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
