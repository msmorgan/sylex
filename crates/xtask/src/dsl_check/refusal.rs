//! What the validator refuses.

use sylex_syl::Span;

/// Why the bundles were refused.
#[derive(Debug, thiserror::Error)]
pub enum CheckError {
    /// `sylex_syl::check` refused them, so nothing was resolved.
    #[error(transparent)]
    Declarations(sylex_syl::check::CheckError),
    /// Every resolution refusal, each also rendered as `path:line:column:
    /// message`.
    #[error("{}", rendered.join("\n"))]
    Resolution {
        refusals: Vec<Refusal>,
        rendered: Vec<String>,
    },
}

impl CheckError {
    #[must_use]
    pub fn count(&self) -> usize {
        match self {
            CheckError::Declarations(error) => error.refusals.len(),
            CheckError::Resolution { refusals, .. } => refusals.len(),
        }
    }
}

/// One refused reference or declaration, located by its span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    pub span: Span,
    pub kind: RefusalKind,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RefusalKind {
    #[error("unknown declaration kind `{0}`")]
    UnknownKind(String),
    #[error("a `{0}` declaration is written with a name")]
    MissingName(String),
    #[error("a `{0}` declaration is written without a name")]
    WrittenName(String),
    #[error("`none` is the word for an omitted value and names no declaration")]
    NamedNone,
    #[error("a `{0}` declaration writes its result sort")]
    MissingSort(String),
    #[error("a `{kind}` declaration is declared at {sort}; it writes no sort")]
    WrittenSort { kind: String, sort: String },
    #[error("a `{0}` declaration has a `{{ ... }}` block")]
    MissingBlock(String),
    #[error("a `{0}` declaration is written as a value, with no header")]
    WrittenHeader(String),
    #[error("a `{0}` declaration is written `{0}(Sort): Sort`")]
    RelationForm(String),
    #[error("a `{kind}` of {sort} to itself; it relates one sort to another")]
    RelationToItself { kind: &'static str, sort: String },
    #[error("duplicate `{kind}` declaration for {subject}")]
    DuplicateRelation { kind: &'static str, subject: String },
    #[error("{member} as a member of {of} closes a cycle: {of} already reaches {member}")]
    MembershipCycle { member: String, of: String },
    #[error("{member} reaches {of} by a second path, through {second} as well as through {first}")]
    SecondPath {
        member: String,
        of: String,
        first: String,
        second: String,
    },
    #[error("`Self` names a sort of an `at` list; this declaration has none")]
    SelfWithoutAtList,
    #[error("a declaration with an `at` list stands at `Self`; write `: Self`")]
    AtListWithoutSelf,
    #[error("a `{0}` declaration takes no `at` list")]
    AtListOnKind(String),
    #[error("{0} is listed twice in `at`")]
    DuplicateAtSort(String),
    #[error(
        "`{word}` is declared at {}, which each reach {target} in {mode} mode",
        .sorts.join(" and ")
    )]
    AmbiguousWord {
        word: String,
        sorts: Vec<String>,
        target: String,
        mode: &'static str,
    },
    #[error("{literal} literal stands at {}, which each reach {target}", .sorts.join(" and "))]
    AmbiguousLiteral {
        literal: String,
        sorts: Vec<String>,
        target: String,
    },
    #[error(
        "`{word}` stands at no sort that holds its arms at {sort}: {}",
        .arms.join(", ")
    )]
    NoCommonSort {
        word: String,
        sort: String,
        arms: Vec<String>,
    },
    #[error("`{word}` at {sort} has a `spelling`; nothing at {sort} prints")]
    SpellingAtTest { word: String, sort: String },
    #[error("`{word}` is declared at the position sort {sort}; declare it at its kind of phrase")]
    PositionDeclaration { word: String, sort: String },
    #[error(
        "`{word}` is a test at {test}, which a card's term cannot write; write the phrase the \
         card prints"
    )]
    TestInCard { word: String, test: String },
    #[error(
        "`nominal` with modifiers of its own in `pre`; write its modifiers in the enclosing \
         `pre` and `post`"
    )]
    ModifiedNominalInPre,
    #[error("duplicate parameter `{0}`")]
    DuplicateParameter(String),
    #[error("unknown sort `{0}`")]
    UnknownSort(String),
    #[error("sort `{0}` takes {1} sort arguments")]
    SortArity(String, usize),
    #[error("duplicate declaration `{name}` at {sort}")]
    DuplicateDeclaration { name: String, sort: String },
    #[error("{0} is reserved for expansion and registry declarations")]
    ReservedConstructor(String),
    #[error("duplicate field `{0}`")]
    DuplicateField(String),
    #[error("unknown field `{field}` at {at}")]
    UnknownField { field: String, at: String },
    #[error("missing field `{field}` at {at}")]
    MissingField { field: String, at: String },
    #[error("unknown parameter `${0}`")]
    UnknownParameter(String),
    #[error("parameter `${name}` has sort {sort}, expected {expected}")]
    ParameterSort {
        name: String,
        sort: String,
        expected: String,
    },
    #[error("`none` stands only at an Option sort, bare; expected {0}")]
    NoneOutsideOption(String),
    #[error("`some` is never written; write the value")]
    SomeWritten,
    #[error("unknown word `{word}` at {sort}")]
    UnknownWord { word: String, sort: String },
    #[error("unknown argument `{argument}` to `{word}`")]
    UnknownArgument { word: String, argument: String },
    #[error("too many arguments to `{0}`")]
    TooManyArguments(String),
    #[error("duplicate argument `{argument}` to `{word}`")]
    DuplicateArgument { word: String, argument: String },
    #[error("missing argument `{argument}` to `{word}`")]
    MissingArgument { word: String, argument: String },
    #[error("required argument `{argument}` to `{word}` is passed by name; pass it positionally")]
    NamedRequired { word: String, argument: String },
    #[error(
        "positional argument to `{0}` binds no required parameter; pass a defaulted argument by \
         name"
    )]
    PositionalDefaulted(String),
    #[error("argument `{argument}` to `{word}` is its declared default; omit it")]
    RedundantDefault { word: String, argument: String },
    #[error(
        "argument `{argument}` to `{word}` is written after `{after}`, which is declared after \
         it; write arguments in declaration order"
    )]
    OutOfOrder {
        word: String,
        argument: String,
        after: String,
    },
    #[error("`triggered` over a single `{0}` condition; write `{0}` at Ability")]
    SingleConditionTriggered(String),
    #[error("empty `{word}` at {sort}; write `{word}` with two or more items")]
    EmptyCoordination { word: String, sort: String },
    #[error("`{word}` at {sort} has one item; write `{instead}` directly")]
    SingleItemCoordination {
        word: String,
        sort: String,
        instead: String,
    },
    #[error(
        "nested `{inner}` inside `{outer}` at TriggerCondition; \
         write one flat `{outer}([...])` list"
    )]
    NestedTriggerCondition { outer: String, inner: String },
    #[error("string at {0}")]
    StringAt(String),
    #[error("integer at {0}")]
    IntegerAt(String),
    #[error("negative integer at Nat")]
    NegativeAtNat,
    #[error("list at {0}")]
    ListAt(String),
    #[error("record at {0}, which is not a record sort")]
    RecordAt(String),
    #[error("a `spelling` is a string literal")]
    SpellingNotString,
    #[error("invalid `spelling`: {0}")]
    InvalidSpelling(sylex_spelling::ErrorKind),
    #[error("invalid `spelling`: unknown parameter `${0}`")]
    SpellingUnknownHole(String),
    #[error("invalid `spelling`: Hole `${name}` states a join, but its sort is {sort}")]
    SpellingJoinNotList { name: String, sort: String },
    #[error("invalid `spelling`: Hole `${name}` at {sort} needs a `#<join>`")]
    SpellingListWithoutJoin { name: String, sort: String },
    #[error(
        "invalid `spelling`: Hole `${name}` stands directly in an optional part, \
         but parameter `{name}` at {sort} is not omissible"
    )]
    SpellingOptionalInomissible { name: String, sort: String },
    #[error(
        "invalid `spelling`: all Holes of `${0}` must stand in one optional part, \
         or all outside every optional part"
    )]
    SpellingSplitParameter(String),
    #[error("invalid `spelling`: an optional part directly holds no Hole")]
    SpellingOptionalWithoutHole,
    #[error(
        "invalid `spelling`: outside every optional part there must be text, a Hole, \
         a Tail or a Gap"
    )]
    SpellingNothingCertain,
    #[error("invalid `spelling`: parameter `{name}` at {sort} has no Hole and is not omissible")]
    SpellingUnprintedParameter { name: String, sort: String },
    #[error("invalid `spelling`: skeleton is ambiguous")]
    SpellingAmbiguousSkeleton,
    #[error("invalid `spelling`: unknown bound {side} `{affix}` in lexicon")]
    SpellingUnknownAffix { affix: String, side: &'static str },
    #[error("shared verb `{0}` needs a `lexical` record with a non-empty `lemma` and `frames`")]
    InvalidSharedVerb(String),
    #[error("`lexical` must be a record")]
    LexicalNotRecord,
    #[error("`lexical.verbs` must be a list")]
    LexicalVerbsNotList,
    #[error(
        "inline verb in `lexical.verbs` needs a non-empty string `lemma` and non-empty `frames`"
    )]
    InvalidLexicalVerb,
    #[error("inline verb `{lemma}` in `lexical` duplicates shared `verb {verb}`")]
    DuplicateSharedVerb { lemma: String, verb: String },
    #[error("`nominal` with neither `pre` nor `post` adds no modifier; write the head")]
    RedundantNominal,
    #[error(
        "`nominal` directly inside `nominal`; combine their `pre` and `post` lists on one `nominal`"
    )]
    NestedNominal,
    #[error("an Agreement postmodifier requires an enclosing plural group determiner")]
    AgreementWithoutGroup,
    #[error(
        "`{word}` at {sort} has no number for the Agreement check; classify it in \
         `dsl_check/nominal.rs`"
    )]
    UnclassifiedNumber { word: String, sort: String },
    #[error("`{0}` stands only in a card face, never in builtin")]
    OwnNameInBuiltin(String),
    #[error("`{word}` requires a card face with a Shortened Name; `{face}` has none")]
    ShortenedNameWithoutShortening { word: String, face: String },
    #[error("dependency cycle: {}", .0.join(" -> "))]
    Cycle(Vec<String>),
}
