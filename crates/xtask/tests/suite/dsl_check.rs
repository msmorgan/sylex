//! `cargo xtask dsl check`: what the resolution validator accepts and refuses.
//!
//! A word that names no declaration at any sort, and a capitalized word outside
//! a sort or a constructor clause, are refused by `sylex_syl` itself
//! (`crates/syl/tests/check.rs` and `parse.rs`); here only that those refusals
//! stop the validator is checked.

use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use sylex_syl::Bundle;
use sylex_syl::FileId;
use xtask::dsl_check::CheckError;
use xtask::dsl_check::RefusalKind;
use xtask::dsl_check::check;

fn bundle(root: &str, first: FileId, sources: &[(&str, &str)]) -> Arc<Bundle> {
    Bundle::from_sources(
        root,
        sources
            .iter()
            .map(|(path, source)| (PathBuf::from(path), (*source).to_owned())),
        first,
    )
    .expect("test sources parse")
}

/// The resolution refusals for a builtin bundle of `sources`, with the text
/// each one spans.
fn refusals(sources: &[(&str, &str)]) -> Vec<(RefusalKind, String)> {
    let builtin = bundle("builtin", FileId(0), sources);
    match check(&builtin, &[]) {
        Ok(_) => Vec::new(),
        Err(CheckError::Declarations(error)) => panic!("sylex_syl refused: {error}"),
        Err(CheckError::Resolution { refusals, .. }) => refusals
            .into_iter()
            .map(|refusal| {
                let file = builtin.file(refusal.span.file).expect("refusal in bundle");
                (refusal.kind, refusal.span.text(&file.source).to_owned())
            })
            .collect(),
    }
}

/// The refusal kinds alone, for one source file.
fn kinds(source: &str) -> Vec<RefusalKind> {
    refusals(&[("a.syl", source)])
        .into_iter()
        .map(|(kind, _)| kind)
        .collect()
}

fn s(text: &str) -> String {
    text.to_owned()
}

const COLORS: &str = "primitive white: Color = White\nprimitive blue: Color = Blue\n";
const PAIR: &str = "template pair(a: Nat, b: Nat = 0): Nat { body = $a }\n";

#[test]
fn the_plugins_resolve() {
    let plugins = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins");
    let builtin = Bundle::load(plugins.join("builtin"), FileId(0)).unwrap();
    let canon = Bundle::load(plugins.join("canon"), builtin.next_file_id()).unwrap();
    let summary = check(&builtin, &[&canon]).unwrap_or_else(|error| panic!("{error}"));
    assert_eq!(summary.files, builtin.files().len() + canon.files().len());
    assert!(summary.declarations["card"] > 0);
}

#[test]
fn resolved_calls_parameters_literals_and_records_pass() {
    let source = format!(
        "{COLORS}{PAIR}\
         template n: Nat {{ body = pair(1) }}\n\
         template m: Nat {{ body = pair(3, b = 2) }}\n\
         template i: Int {{ body = -4 }}\n\
         template c: Option(Color) {{ body = white }}\n\
         template o: Option(Color) {{ body = none }}\n\
         template p(x: Color): Option(Color) {{ body = $x }}\n\
         template l: List(Color) {{ body = [white, blue] }}\n\
         template r: EventOrdinal {{ body = {{ ordinal = first }} }}\n\
         primitive first: Ordinal = First\n"
    );
    assert_eq!(kinds(&source), []);
}

#[test]
fn a_word_with_no_declaration_at_the_expected_sort_is_refused() {
    let source = format!("{COLORS}template t: Stat {{ body = white }}");
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [(
            RefusalKind::UnknownWord {
                word: s("white"),
                sort: s("Stat")
            },
            s("white")
        )]
    );
}

#[test]
fn a_parameter_at_another_sort_is_refused() {
    let source = format!(
        "{COLORS}template t(x: Color): Nat {{ body = $x }}\n\
         template u(x: Color): Option(Nat) {{ body = $x }}"
    );
    assert_eq!(
        kinds(&source),
        [
            RefusalKind::ParameterSort {
                name: s("x"),
                sort: s("Color"),
                expected: s("Nat")
            },
            RefusalKind::ParameterSort {
                name: s("x"),
                sort: s("Color"),
                expected: s("Option(Nat)")
            },
        ]
    );
}

#[test]
fn an_unknown_parameter_and_a_default_reading_a_later_one_are_refused() {
    assert_eq!(
        kinds(
            "template t(x: Int): Int { body = $y }\n\
             template u(early: Int = $later, later: Int): Int { body = $early }"
        ),
        [
            RefusalKind::UnknownParameter(s("y")),
            RefusalKind::UnknownParameter(s("later")),
        ]
    );
}

#[test]
fn wrong_arity_is_refused() {
    let source = format!(
        "{PAIR}\
         template missing: Nat {{ body = pair }}\n\
         template over: Nat {{ body = pair(1, b = 2, 3) }}\n\
         template unknown: Nat {{ body = pair(1, c = 2) }}\n\
         template twice: Nat {{ body = pair(1, b = 2, b = 3) }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [
            (
                RefusalKind::MissingArgument {
                    word: s("pair"),
                    argument: s("a")
                },
                s("pair")
            ),
            (RefusalKind::TooManyArguments(s("pair")), s("3")),
            (
                RefusalKind::UnknownArgument {
                    word: s("pair"),
                    argument: s("c")
                },
                s("c")
            ),
            (
                RefusalKind::DuplicateArgument {
                    word: s("pair"),
                    argument: s("b")
                },
                s("b = 3")
            ),
        ]
    );
}

/// `nominal`-shaped: an optional parameter declared before a required one,
/// so a call reads in printed order.
const FRAMED: &str = "template framed(pre: Nat = 0, x: Nat, post: Nat = 0): Nat { body = $x }\n";

#[test]
fn a_positional_argument_binds_the_next_required_parameter() {
    let source = format!(
        "{FRAMED}\
         template bare: Nat {{ body = framed(1) }}\n\
         template both: Nat {{ body = framed(pre = 2, 1, post = 3) }}\n\
         template after: Nat {{ body = framed(1, post = 3) }}"
    );
    assert_eq!(kinds(&source), []);
}

#[test]
fn a_required_argument_passed_by_name_is_refused() {
    let source = format!("{PAIR}template t: Nat {{ body = pair(a = 1, b = 2) }}");
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [(
            RefusalKind::NamedRequired {
                word: s("pair"),
                argument: s("a")
            },
            s("a = 1")
        )]
    );
}

#[test]
fn a_defaulted_argument_passed_positionally_is_refused() {
    let source = format!("{FRAMED}template t: Nat {{ body = framed(1, 2) }}");
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [(RefusalKind::PositionalDefaulted(s("framed")), s("2"))]
    );
}

/// A positional argument past the required parameters binds nothing, so it
/// is refused once and never resolved: no order or sort refusal follows.
#[test]
fn surplus_positional_arguments_on_an_optional_first_declaration_are_refused_once() {
    let source = format!(
        "{COLORS}{FRAMED}\
         template t: Nat {{ body = framed(1, white, 3, 4) }}\n\
         template u: Nat {{ body = framed(pre = 2, 1, 3, post = 4, 5) }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [
            (RefusalKind::PositionalDefaulted(s("framed")), s("white")),
            (RefusalKind::PositionalDefaulted(s("framed")), s("3")),
            (RefusalKind::TooManyArguments(s("framed")), s("4")),
            (RefusalKind::TooManyArguments(s("framed")), s("3")),
            (RefusalKind::TooManyArguments(s("framed")), s("5")),
        ]
    );
}

#[test]
fn arguments_out_of_declaration_order_are_refused() {
    let source = format!(
        "{FRAMED}\
         template late: Nat {{ body = framed(1, pre = 2) }}\n\
         template swapped: Nat {{ body = framed(post = 3, 1) }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [
            (
                RefusalKind::OutOfOrder {
                    word: s("framed"),
                    argument: s("pre"),
                    after: s("x")
                },
                s("pre = 2")
            ),
            (
                RefusalKind::OutOfOrder {
                    word: s("framed"),
                    argument: s("x"),
                    after: s("post")
                },
                s("1")
            ),
        ]
    );
}

#[test]
fn a_named_argument_equal_to_its_default_is_refused() {
    let source = format!(
        "{COLORS}{PAIR}\
         template list(x: Nat, cs: List(Color) = [white, blue]): Nat {{ body = $x }}\n\
         template echo(x: Nat, y: Nat = $x): Nat {{ body = $y }}\n\
         template zero: Nat {{ body = pair(1, b = 0) }}\n\
         template laidOut: Nat {{ body = list(1, cs = [\n  white,\n  blue]) }}\n\
         template reread: Nat {{ body = echo(4, y = 4) }}\n\
         template differs: Nat {{ body = list(1, cs = [blue, white]) }}\n\
         template other: Nat {{ body = echo(4, y = 5) }}"
    );
    let redundant = |word: &str, argument: &str| RefusalKind::RedundantDefault {
        word: s(word),
        argument: s(argument),
    };
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [
            (redundant("pair", "b"), s("b = 0")),
            (redundant("list", "cs"), s("cs = [\n  white,\n  blue]")),
            (redundant("echo", "y"), s("y = 4")),
        ]
    );
}

#[test]
fn none_stands_only_at_an_option_and_some_is_never_written() {
    assert_eq!(
        kinds(
            "template some(x: Int): Int { body = $x }\n\
             template n: Int { body = none }\n\
             template s: Option(Int) { body = some(1) }\n\
             template ok: Int { body = some(1) }"
        ),
        [
            RefusalKind::NoneOutsideOption(s("Int")),
            RefusalKind::SomeWritten,
        ]
    );
}

#[test]
fn literals_lists_and_records_stand_only_at_their_own_sorts() {
    let source = format!(
        "{COLORS}\
         template a: Color {{ body = \"white\" }}\n\
         template b: Color {{ body = 1 }}\n\
         template c: Nat {{ body = -1 }}\n\
         template d: Color {{ body = [white] }}\n\
         template e: Color {{ body = {{ name = \"x\" }} }}"
    );
    assert_eq!(
        kinds(&source),
        [
            RefusalKind::StringAt(s("Color")),
            RefusalKind::IntegerAt(s("Color")),
            RefusalKind::NegativeAtNat,
            RefusalKind::ListAt(s("Color")),
            RefusalKind::RecordAt(s("Color")),
        ]
    );
}

#[test]
fn record_fields_are_checked_against_the_record_sort() {
    assert_eq!(
        kinds(
            "primitive first: Ordinal = First\n\
             template a: EventOrdinal { body = { per = none } }\n\
             template b: EventOrdinal { body = { ordinal = first, color = first } }\n\
             template c: EventOrdinal { body = { ordinal = first, ordinal = first } }"
        ),
        [
            RefusalKind::MissingField {
                field: s("ordinal"),
                at: s("EventOrdinal")
            },
            RefusalKind::UnknownField {
                field: s("color"),
                at: s("EventOrdinal")
            },
            RefusalKind::DuplicateField(s("ordinal")),
        ]
    );
}

#[test]
fn declaration_blocks_are_checked_against_their_kind() {
    assert_eq!(
        kinds(
            "primitive p: Color = White { body = 1 }\n\
             template t: Int { }\n\
             designation monarch { }\n\
             template u: Int { body = 1, body = 2 }\n\
             template v: Int { body = 1, identity = \"actor\" }"
        ),
        [
            RefusalKind::UnknownField {
                field: s("body"),
                at: s("`primitive` p")
            },
            RefusalKind::MissingField {
                field: s("body"),
                at: s("`template` t")
            },
            RefusalKind::MissingField {
                field: s("scope"),
                at: s("`designation` monarch")
            },
            RefusalKind::DuplicateField(s("body")),
            RefusalKind::UnknownField {
                field: s("identity"),
                at: s("`template` v")
            },
        ]
    );
}

/// `permanentType` is required on a card type, `false` included [CR#110.4].
#[test]
fn a_card_type_without_permanent_type_is_refused() {
    assert_eq!(
        kinds(
            "primitive true: Bool = True\n\
             card_type kindred { }\n\
             card_type creature { permanentType = true }"
        ),
        [RefusalKind::MissingField {
            field: s("permanentType"),
            at: s("`card_type` kindred")
        }]
    );
}

/// A supertype is a catalog declaration, as a card type is: it declares its
/// name at `Supertype`, where no primitive builds a constructor, and its block
/// holds only a supertype's fields [CR#205.4a].
#[test]
fn a_supertype_is_a_catalog_declaration() {
    assert_eq!(
        refusals(&[(
            "a.syl",
            "primitive yes: Nominal = Yes\n\
             primitive has(supertype: Supertype): Nominal = HasSupertype\n\
             supertype legendary { name = \"Legendary\", confers = [] }\n\
             supertype basic\n\
             supertype world { permanentType = yes }\n\
             primitive snow: Supertype = Snow\n\
             template t: Nominal { body = has(legendary) }"
        )]),
        [
            (RefusalKind::MissingBlock(s("supertype")), s("basic")),
            (
                RefusalKind::ReservedConstructor(s("Supertype.Snow")),
                s("Snow")
            ),
            (
                RefusalKind::UnknownField {
                    field: s("permanentType"),
                    at: s("`supertype` world")
                },
                s("permanentType")
            ),
        ]
    );
}

/// A rules row's fields are its `body`, a record of its sort, written one
/// way: never directly in the block.
#[test]
fn a_rules_row_writes_its_record_as_its_body() {
    let preamble = "primitive yes: Nominal = Yes\n\
                    primitive always: Condition = Always\n\
                    primitive nothing: VerbPhrase = Nothing\n";
    let source = format!(
        "{preamble}\
         sba a {{ body = {{ scope = yes, when = always, then = nothing }} }}\n\
         sba b {{ scope = yes }}\n\
         sba c {{ body = {{ scope = yes, when = always }} }}"
    );
    assert_eq!(
        kinds(&source),
        [
            RefusalKind::UnknownField {
                field: s("scope"),
                at: s("`sba` b")
            },
            RefusalKind::MissingField {
                field: s("body"),
                at: s("`sba` b")
            },
            RefusalKind::MissingField {
                field: s("then"),
                at: s("SbaRule")
            },
        ]
    );
}

/// A top-level rules row is a catalog declaration that card text can name, as
/// it names the "legend rule" [CR#205.4d]: it takes `lexical` and `spelling`,
/// a reference to it is no dependency edge, and no primitive builds a
/// constructor at its sort.
#[test]
fn a_rules_row_is_a_nameable_catalog_declaration() {
    let preamble = "primitive yes: Nominal = Yes\n\
                    primitive always: Condition = Always\n\
                    primitive nothing: VerbPhrase = Nothing\n\
                    primitive property(spec: StaticSpec): Conferral = Property\n\
                    primitive exempt(rule: SbaRule): StaticSpec = Exempt\n\
                    primitive grants(rule: ConferralRule): StaticSpec = Grants\n";
    let source = format!(
        "{preamble}\
         sba a {{\n\
           lexical = {{lemma = \"a rule\"}},\n\
           spelling = \"a rule\",\n\
           body = {{ scope = yes, when = always, then = nothing }}\n\
         }}\n\
         sba b {{\n\
           lexical = {{lemma = \"b rule\"}},\n\
           body = {{ scope = yes, when = always, then = nothing }}\n\
         }}\n\
         conferral c {{\n\
           lexical = {{lemma = \"c rule\"}},\n\
           spelling = \"c rule\",\n\
           body = {{ scope = yes, confer = property(grants(c)) }}\n\
         }}\n\
         template t: StaticSpec {{ body = exempt(a) }}\n\
         primitive raw: SbaRule = Raw\n\
         primitive rawConferral: ConferralRule = Raw"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [
            (RefusalKind::ReservedConstructor(s("SbaRule.Raw")), s("Raw")),
            (
                RefusalKind::ReservedConstructor(s("ConferralRule.Raw")),
                s("Raw")
            ),
        ]
    );
}

#[test]
fn a_token_declares_its_characteristics_and_any_back_face() {
    let source = format!(
        "{COLORS}\
         template make(spec: TokenSpec): Nat {{ body = 0 }}\n\
         primitive defined(token: DefinedToken): TokenSpec = Defined\n\
         token single {{ body = {{ characteristics = {{ colors = [blue] }} }} }}\n\
         token double {{ body = {{ characteristics = {{}}, back = {{ name = \"Back\" }} }} }}\n\
         token wrong {{ body = {{ back = {{ colors = [white] }}, face = {{}} }} }}\n\
         template a: Nat {{ body = make(defined(single)) }}\n\
         template b: Nat {{ body = make(single) }}"
    );
    assert_eq!(
        kinds(&source),
        [
            RefusalKind::UnknownField {
                field: s("face"),
                at: s("DefinedToken")
            },
            RefusalKind::MissingField {
                field: s("characteristics"),
                at: s("DefinedToken")
            },
            RefusalKind::UnknownWord {
                word: s("single"),
                sort: s("TokenSpec")
            },
        ]
    );
}

#[test]
fn a_defined_tokens_subtypes_are_read_by_a_predicate_over_the_token() {
    assert_eq!(
        kinds(
            "primitive hasSubtypesOf(token: DefinedToken): Nominal = HasSubtypesOf\n\
             primitive yes: Nominal = Yes\n\
             token jaceToken { body = { characteristics = {} } }\n\
             template a: Nominal { body = hasSubtypesOf(jaceToken) }\n\
             template b: Nominal { body = hasSubtypesOf(yes) }\n\
             primitive subtypeOf(token: DefinedToken): Subtype = SubtypeOf"
        ),
        [
            RefusalKind::ReservedConstructor(s("Subtype.SubtypeOf")),
            RefusalKind::UnknownWord {
                word: s("yes"),
                sort: s("DefinedToken")
            },
        ]
    );
}

#[test]
fn headers_are_checked_against_their_kind() {
    assert_eq!(
        kinds(
            "widget w { }\n\
             template { body = 1 }\n\
             card c { name = \"C\" }\n\
             template none: Int { body = 1 }\n\
             template t { body = 1 }\n\
             relation paired: RelationLabel { }\n\
             relation haunting\n\
             template d(x: Int, x: Int): Int { body = 1 }"
        ),
        [
            RefusalKind::UnknownKind(s("widget")),
            RefusalKind::MissingName(s("template")),
            RefusalKind::WrittenName(s("card")),
            RefusalKind::NamedNone,
            RefusalKind::MissingSort(s("template")),
            RefusalKind::WrittenSort {
                kind: s("relation"),
                sort: s("RelationLabel")
            },
            RefusalKind::MissingBlock(s("relation")),
            RefusalKind::DuplicateParameter(s("x")),
        ]
    );
}

#[test]
fn sort_constructors_take_one_sort_argument_and_others_none() {
    assert_eq!(
        kinds(
            "template a(x: List): Int { body = 1 }\n\
             template b(x: Option(Int, Int)): Int { body = 1 }\n\
             template c: Int(Nat) { body = 1 }"
        ),
        [
            RefusalKind::SortArity(s("List"), 1),
            RefusalKind::SortArity(s("Option"), 1),
            RefusalKind::SortArity(s("Int"), 0),
        ]
    );
}

#[test]
fn a_sort_no_declaration_names_is_refused() {
    assert_eq!(
        refusals(&[(
            "a.syl",
            "primitive yes: Bool = Yes\n\
             template t(x: Option(Bol) = none): Bool { body = yes }"
        )]),
        [(RefusalKind::UnknownSort(s("Bol")), s("Bol"))]
    );
}

#[test]
fn a_sort_only_produced_is_refused() {
    assert_eq!(
        refusals(&[(
            "a.syl",
            "primitive x: Actoin = X\n\
             primitive y: List(Actoin) = Y"
        )]),
        [
            (RefusalKind::UnknownSort(s("Actoin")), s("Actoin")),
            (RefusalKind::UnknownSort(s("Actoin")), s("Actoin")),
        ]
    );
}

/// A sort misspelled the same way where it is produced and where it is
/// consumed is a sort to the check: only its name is known.
#[test]
fn a_sort_produced_and_consumed_is_known_even_when_misspelled_alike() {
    assert_eq!(
        kinds(
            "primitive x: Actoin = X\n\
             template t(a: Actoin): Actoin { body = $a }"
        ),
        []
    );
}

#[test]
fn two_declarations_at_one_name_and_sort_are_refused() {
    assert_eq!(
        kinds(
            "template item: Nat { body = 1 }\n\
             template item: Nat { body = 2 }\n\
             template item: Int { body = 2 }\n\
             ability kicker { }\n\
             template kicker: KeywordLabel { body = kicker }\n\
             primitive nothing: VerbPhrase = Nothing\n\
             action scry { body = nothing }\n\
             template scry: Deed { body = scry }"
        ),
        [
            RefusalKind::DuplicateDeclaration {
                name: s("item"),
                sort: s("Nat")
            },
            RefusalKind::DuplicateDeclaration {
                name: s("kicker"),
                sort: s("KeywordLabel")
            },
            RefusalKind::DuplicateDeclaration {
                name: s("scry"),
                sort: s("Deed")
            },
        ]
    );
}

#[test]
fn an_action_declares_a_deed_unless_it_is_deedless() {
    let preamble = "primitive nothing: VerbPhrase = Nothing\nability crew { }\n";
    let source = format!(
        "{preamble}\
         action scry {{ body = nothing }}\n\
         action wither {{ body = nothing, deed = none }}\n\
         action crewing {{ body = nothing, ofAbility = crew }}\n\
         template a: Deed {{ body = scry }}\n\
         template b: Deed {{ body = wither }}\n\
         template c: Deed {{ body = crewing }}"
    );
    assert_eq!(
        kinds(&source),
        [
            RefusalKind::UnknownWord {
                word: s("wither"),
                sort: s("Deed")
            },
            RefusalKind::UnknownWord {
                word: s("crewing"),
                sort: s("Deed")
            },
        ]
    );
}

#[test]
fn a_primitive_building_a_reserved_constructor_is_refused() {
    assert_eq!(
        refusals(&[(
            "a.syl",
            "primitive raw(name: String): CounterKind = Named\n\
             primitive mention(id: String, body: NounPhrase): NounPhrase = FirstMention\n\
             primitive caller(body: Cost): Cost = InCaller\n\
             primitive deed(label: String): Deed = Action\n\
             primitive label(name: String): KeywordLabel = Named"
        )]),
        [
            (
                RefusalKind::ReservedConstructor(s("CounterKind.Named")),
                s("Named")
            ),
            (
                RefusalKind::ReservedConstructor(s("NounPhrase.FirstMention")),
                s("FirstMention")
            ),
            (
                RefusalKind::ReservedConstructor(s("Cost.InCaller")),
                s("InCaller")
            ),
            (
                RefusalKind::ReservedConstructor(s("Deed.Action")),
                s("Action")
            ),
            (
                RefusalKind::ReservedConstructor(s("KeywordLabel.Named")),
                s("Named")
            ),
        ]
    );
}

#[test]
fn only_the_deed_constructor_an_action_writes_is_reserved_at_deed() {
    assert_eq!(
        kinds(
            "primitive target: CoreDeed = Target\n\
             primitive core(deed: CoreDeed): Deed = Core\n\
             primitive raw(label: String): Deed = Action"
        ),
        [RefusalKind::ReservedConstructor(s("Deed.Action"))]
    );
}

/// A Primitive Keyword Ability's label is a primitive at `KeywordLabel` that
/// builds the constructor its own name capitalizes, with no parameters; it
/// stands where a label is expected, as an `ability` declaration's does.
#[test]
fn a_keyword_label_primitive_builds_only_its_own_name() {
    assert_eq!(
        kinds(
            "primitive firstStrike: KeywordLabel = FirstStrike\n\
             ability flying { }\n\
             template a: KeywordLabel { body = firstStrike }\n\
             template b: KeywordLabel { body = flying }"
        ),
        []
    );
    assert_eq!(
        kinds(
            "primitive firstStrike: KeywordLabel = Strike\n\
             primitive trample(over: String): KeywordLabel = Trample\n\
             ability flying { }\n\
             primitive flying: KeywordLabel = Flying"
        ),
        [
            RefusalKind::ReservedConstructor(s("KeywordLabel.Strike")),
            RefusalKind::ReservedConstructor(s("KeywordLabel.Trample")),
            RefusalKind::DuplicateDeclaration {
                name: s("flying"),
                sort: s("KeywordLabel")
            },
        ]
    );
}

const RING: &str = "primitive ownScope(deed: Option(Deed), body: VerbPhrase): VerbPhrase = OwnScope\n\
                    primitive nothing: VerbPhrase = Nothing\n\
                    action theRingTemptsYou { body = ownScope(theRingTemptsYou, nothing) }";

#[test]
fn only_the_rings_file_may_build_its_own_scope() {
    let ring = "definitions/keyword_action/theRingTemptsYou.syl";
    assert_eq!(refusals(&[(ring, RING)]), []);
    assert_eq!(
        refusals(&[
            (ring, RING),
            (
                "elsewhere.syl",
                "template t: VerbPhrase { body = ownScope(none, nothing) }"
            )
        ]),
        [(
            RefusalKind::ReservedConstructor(s("VerbPhrase.OwnScope")),
            s("ownScope")
        )]
    );
    // Elsewhere the declaration is refused; its calls are not refused again.
    assert_eq!(
        kinds(RING),
        [RefusalKind::ReservedConstructor(s("VerbPhrase.OwnScope"))]
    );
    // Only the builtin bundle's file is the Ring's.
    let builtin = bundle(
        "builtin",
        FileId(0),
        &[("a.syl", "primitive nothing: VerbPhrase = Nothing")],
    );
    let other = bundle(
        "builtin",
        builtin.next_file_id(),
        &[(
            ring,
            "primitive ownScope(deed: Option(Deed), body: VerbPhrase): VerbPhrase = OwnScope",
        )],
    );
    let Err(CheckError::Resolution { refusals, .. }) = check(&builtin, &[&other]) else {
        panic!("the other bundle's Ring file is refused");
    };
    assert_eq!(
        refusals[0].kind,
        RefusalKind::ReservedConstructor(s("VerbPhrase.OwnScope"))
    );
}

#[test]
fn dependency_cycles_are_refused_but_catalog_references_are_no_edge() {
    assert_eq!(
        kinds(
            "template a: Int { body = b }\n\
             template b: Int { body = a }\n\
             template c: Int { body = c }\n\
             primitive yes: Predicate = Yes\n\
             primitive grant(scope: Predicate, type: CardType): Conferral = Grant\n\
             primitive true: Bool = True\n\
             primitive false: Bool = False\n\
             card_type battle { permanentType = true, confers = [grant(yes, battle)] }"
        ),
        [
            RefusalKind::Cycle(vec![s("a"), s("b")]),
            RefusalKind::Cycle(vec![s("c")]),
        ]
    );
}

#[test]
fn a_single_faced_card_is_an_explicit_call() {
    let source = format!(
        "{COLORS}primitive singleFaced(face: Characteristics): Card = SingleFaced\n\
         card singleFaced({{ name = \"Card\", colors = [white] }})"
    );
    assert_eq!(kinds(&source), []);
}

#[test]
fn a_record_at_card_is_refused() {
    let source = format!("{COLORS}card {{ name = \"Card\", colors = [white] }}");
    assert_eq!(kinds(&source), [RefusalKind::RecordAt(s("Card"))]);
}

#[test]
fn a_characteristics_wrap_on_a_card_face_is_refused() {
    let source = format!(
        "{COLORS}primitive singleFaced(face: Characteristics): Card = SingleFaced\n\
         card singleFaced({{ characteristics = {{ name = \"Card\", colors = [white] }} }})"
    );
    assert_eq!(
        kinds(&source),
        [RefusalKind::UnknownField {
            field: s("characteristics"),
            at: s("Characteristics")
        }]
    );
}

#[test]
fn a_card_face_is_a_characteristics_record() {
    let source = format!(
        "{COLORS}primitive transforming(front: Characteristics, back: Characteristics): Card = Transforming\n\
         card transforming({{ name = \"F\", colors = [white] }}, {{ name = \"B\", colors = [1] }})"
    );
    assert_eq!(kinds(&source), [RefusalKind::IntegerAt(s("Color"))]);
}

/// A subtype's `types` is the list of card types it is correlated with,
/// written out whole: Spirit's `[creature, kindred]` [CR#205.3m], Arcane's
/// `[instant, sorcery]` [CR#205.3k], Siege's `[battle]`. A lone card type is
/// no list.
#[test]
fn a_subtype_types_is_a_list_of_card_types() {
    assert_eq!(
        refusals(&[(
            "a.syl",
            "primitive true: Bool = True\n\
             primitive false: Bool = False\n\
             card_type creature { permanentType = true }\n\
             card_type kindred { permanentType = false }\n\
             card_type instant { permanentType = false }\n\
             card_type sorcery { permanentType = false }\n\
             card_type battle { permanentType = true }\n\
             subtype spirit { types = [creature, kindred] }\n\
             subtype arcane { types = [instant, sorcery] }\n\
             subtype siege { types = [battle] }\n\
             subtype bad { types = battle }"
        )]),
        [(
            RefusalKind::UnknownWord {
                word: s("battle"),
                sort: s("List(CardType)")
            },
            s("battle")
        )]
    );
}

/// No word at `List(CardType)` stands for several card types: *Grizzly
/// Bears*'s `types = creature` is refused beside the real builtin bundle
/// rather than read as a kindred creature.
#[test]
fn a_card_type_word_is_no_list_of_card_types() {
    assert_eq!(
        plugin_refusals(
            "card singleFaced({\n\
               name = \"Grizzly Bears\",\n\
               types = creature})"
        ),
        [(
            RefusalKind::UnknownWord {
                word: s("creature"),
                sort: s("List(CardType)")
            },
            s("creature")
        )]
    );
}

#[test]
fn sylex_syl_refusals_stop_the_check() {
    let builtin = bundle(
        "builtin",
        FileId(0),
        &[("a.syl", "template t: Int { body = nowhere }")],
    );
    let Err(CheckError::Declarations(error)) = check(&builtin, &[]) else {
        panic!("sylex_syl refuses `nowhere`");
    };
    assert_eq!(error.refusals.len(), 1);
}

#[test]
fn any_declared_card_form_resolves() {
    assert_eq!(
        kinds(
            "primitive meld(front: Characteristics, back: Characteristics): Card = Meld\n\
             card meld({ name = \"Bruna\" }, { name = \"Brisela\" })"
        ),
        []
    );
}

/// The resolution refusals for `cards` checked as a bundle beside the real
/// builtin bundle, with the text each one spans.
fn plugin_refusals(cards: &str) -> Vec<(RefusalKind, String)> {
    let plugins = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins");
    let builtin = Bundle::load(plugins.join("builtin"), FileId(0)).unwrap();
    let canon = bundle("canon", builtin.next_file_id(), &[("cards.syl", cards)]);
    match check(&builtin, &[&canon]) {
        Ok(_) => Vec::new(),
        Err(CheckError::Declarations(error)) => panic!("sylex_syl refused: {error}"),
        Err(CheckError::Resolution { refusals, .. }) => refusals
            .into_iter()
            .map(|refusal| {
                let file = canon.file(refusal.span.file).expect("refusal in the cards");
                (refusal.kind, refusal.span.text(&file.source).to_owned())
            })
            .collect(),
    }
}

/// "At" names a turn point and "when" and "whenever" an event [CR#603.2b]:
/// *Elvish Visionary*'s enters trigger under `at`, and *Thallid*'s upkeep
/// trigger under `whenever` and *Myr Prototype*'s under `when`, are refused
/// at the trigger word's argument: each refusal spans the head word of that
/// argument, which each card writes once.
#[test]
fn at_refuses_an_event_and_whenever_refuses_a_turn_point() {
    assert_eq!(
        plugin_refusals(
            "card singleFaced({\n\
               name = \"Elvish Visionary\",\n\
               types = [creature],\n\
               text = [at(does(actor(thisCreature), enter), draw(lit(1)))]})\n\
             card singleFaced({\n\
               name = \"Thallid\",\n\
               types = [creature],\n\
               text = [\n\
                 whenever(\n\
                   beginningOfYour(upkeep),\n\
                   putCounters(lit(1), printedKind(sporeCounter), thisCreature))]})\n\
             card singleFaced({\n\
               name = \"Myr Prototype\",\n\
               types = [artifact, creature],\n\
               text = [\n\
                 when(\n\
                   beginningOfYour(upkeep),\n\
                   putCounters(lit(1), printedKind(p1p1Counter), thisCreature))]})"
        ),
        [
            (
                RefusalKind::UnknownWord {
                    word: s("does"),
                    sort: s("TurnPoint")
                },
                s("does")
            ),
            (
                RefusalKind::UnknownWord {
                    word: s("beginningOfYour"),
                    sort: s("GameEvent")
                },
                s("beginningOfYour")
            ),
            (
                RefusalKind::UnknownWord {
                    word: s("beginningOfYour"),
                    sort: s("GameEvent")
                },
                s("beginningOfYour")
            ),
        ]
    );
}

/// Joined trigger conditions are one disjunction of trigger conditions
/// [CR#603.1b], each printing its own trigger word, in either order and with
/// any of the three words. Most cards print "and" between them (the Template
/// `and` over `or`): *Shrine of Limitless Power*'s "At the beginning of your
/// upkeep and whenever you cast a black spell", *Minsc & Boo, Timeless
/// Heroes*'s "When Minsc & Boo enters and at the beginning of your upkeep" and
/// *Bartered Cow*'s "When this creature dies and when you discard this card"
/// resolve. *Tenuous Truce* prints "or" (the primitive `or`): "When you attack
/// enchanted opponent or a planeswalker they control or when they attack you or
/// a planeswalker you control", here with its first arm shortened to "when you
/// attack enchanted player". An event is no trigger condition without its
/// trigger word, so *Bartered Cow* written with a bare second arm is refused at
/// that arm's head word.
#[test]
fn joined_trigger_conditions_are_one_disjunction() {
    assert_eq!(
        plugin_refusals(
            "card singleFaced({\n\
               name = \"Shrine of Limitless Power\",\n\
               types = [artifact],\n\
               text = [\n\
                 triggered(\n\
                   and([\n\
                     at(beginningOfYour(upkeep)),\n\
                     whenever(does(actor(you), cast(a(nominal(pre = [black], spell)))))]),\n\
                   putCounters(lit(1), printedKind(chargeCounter), thisArtifact))]})\n\
             card singleFaced({\n\
               name = \"Minsc & Boo, Timeless Heroes\",\n\
               types = [planeswalker],\n\
               text = [\n\
                 triggered(\n\
                   and([when(does(actor(thisPlaneswalker), enter)), at(beginningOfYour(upkeep))]),\n\
                   may(\n\
                     create(\n\
                       lit(1),\n\
                       written({\n\
                         characteristics = {\n\
                           name = \"Boo\",\n\
                           colors = [red],\n\
                           supertypes = [legendary],\n\
                           types = [creature],\n\
                           subtypes = [hamster],\n\
                           text = [trample, haste],\n\
                           power = lit(1),\n\
                           toughness = lit(1)}}))))]})\n\
             card singleFaced({\n\
               name = \"Bartered Cow\",\n\
               types = [creature],\n\
               text = [\n\
                 triggered(\n\
                   and([when(does(actor(thisCreature), die)), when(does(actor(you), discard(thisCard)))]),\n\
                   create(lit(1), defined(foodToken)))]})\n\
             card singleFaced({\n\
               name = \"Tenuous Truce\",\n\
               types = [enchantment],\n\
               subtypes = [aura],\n\
               text = [\n\
                 triggered(\n\
                   or([\n\
                     when(does(actor(you), attack(defender = enchanted(player)))),\n\
                     when(\n\
                       does(\n\
                         actor(enchanted(player)),\n\
                         attack(defender = or([you, a(nominal(planeswalker, post = [youControl]))]))))]),\n\
                   sacrifice(thisAura))]})\n\
             card singleFaced({\n\
               name = \"Bartered Cow\",\n\
               types = [creature],\n\
               text = [\n\
                 triggered(\n\
                   and([when(does(actor(thisCreature), die)), does(actor(you), discard(thisCard))]),\n\
                   create(lit(1), defined(foodToken)))]})"
        ),
        [(
            RefusalKind::UnknownWord {
                word: s("does"),
                sort: s("TriggerCondition")
            },
            s("does")
        )]
    );
}

/// Three coercions over their primitives.
const COERCIONS: &str = "primitive type(type: CardType): Nominal = Type\n\
     primitive subtype(subtype: Subtype): Nominal = Subtype\n\
     primitive lit(value: Int): Amount = Literal\n\
     primitive noun(word: Nominal): Condition = Noun\n\
     coercion(type: CardType): Nominal { body = type($type) }\n\
     coercion(subtype: Subtype): Nominal { body = subtype($subtype) }\n\
     coercion(value: Int): Amount { body = lit($value) }\n\
     primitive true: Bool = True\n\
     card_type creature { permanentType = true }\n\
     subtype goblin { types = [creature] }\n";

#[test]
fn a_coercion_applies_at_a_sort_mismatch() {
    let source = format!(
        "{COERCIONS}\
         template word: Nominal {{ body = creature }}\n\
         template sub: Nominal {{ body = goblin }}\n\
         template int: Amount {{ body = -2 }}\n\
         template param(t: CardType): Nominal {{ body = $t }}\n\
         template items: List(Nominal) {{ body = [creature, goblin] }}\n\
         template wrapped: Nominal {{ body = type(creature) }}\n\
         template literal: Amount {{ body = lit(3) }}\n\
         template own: Int {{ body = 3 }}\n\
         template arg: Condition {{ body = noun(goblin) }}"
    );
    assert_eq!(kinds(&source), []);
}

/// The redundant-default refusal looks through a coercion: `n = 0` is the
/// default `lit(0)`, and `w = type(creature)` the default `creature`.
#[test]
fn a_named_argument_equal_to_its_default_through_a_coercion_is_refused() {
    let source = format!(
        "{COERCIONS}\
         template amt(x: Int, n: Amount = lit(0)): Int {{ body = $x }}\n\
         template noun(x: Int, w: Nominal = creature): Int {{ body = $x }}\n\
         template bare: Int {{ body = amt(1, n = 0) }}\n\
         template wrapped: Int {{ body = noun(1, w = type(creature)) }}\n\
         template other: Int {{ body = amt(1, n = 1) }}\n\
         template sub: Int {{ body = noun(1, w = goblin) }}"
    );
    let redundant = |word: &str, argument: &str| RefusalKind::RedundantDefault {
        word: s(word),
        argument: s(argument),
    };
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [
            (redundant("amt", "n"), s("n = 0")),
            (redundant("noun", "w"), s("w = type(creature)")),
        ]
    );
}

/// A value reached through a coercion keeps its arguments bound by name:
/// `sq(1)` at Amount is the default `lit(sq(1))`.
#[test]
fn a_named_argument_equal_to_its_default_through_a_coerced_call_is_refused() {
    let source = format!(
        "{COERCIONS}\
         template sq(n: Int, k: Int = 0): Int {{ body = $n }}\n\
         template h(x: Int, y: Amount = lit(sq(1))): Int {{ body = $x }}\n\
         template g(x: Int, y: Amount = sq(1)): Int {{ body = $x }}\n\
         template a: Int {{ body = h(1, y = sq(1)) }}\n\
         template b: Int {{ body = g(1, y = lit(sq(1))) }}\n\
         template c: Int {{ body = h(1, y = sq(2)) }}\n\
         template d: Int {{ body = g(1, y = sq(1, k = 2)) }}"
    );
    let redundant = |word: &str, argument: &str| RefusalKind::RedundantDefault {
        word: s(word),
        argument: s(argument),
    };
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [
            (redundant("h", "y"), s("y = sq(1)")),
            (redundant("g", "y"), s("y = lit(sq(1))")),
        ]
    );
}

/// A record's fields are compared at their sorts, so a coercion inside one
/// is looked through.
#[test]
fn a_named_record_argument_equal_to_its_default_is_refused() {
    let source = "primitive lit(value: Int): Amount = Literal\n\
                  coercion(value: Int): Amount { body = lit($value) }\n\
                  template f(x: Int, c: Characteristics = { name = \"a\", power = lit(2) }): Int { body = $x }\n\
                  template same: Int { body = f(1, c = { power = 2, name = \"a\" }) }\n\
                  template other: Int { body = f(1, c = { name = \"a\", power = 3 }) }";
    assert_eq!(
        refusals(&[("a.syl", source)]),
        [(
            RefusalKind::RedundantDefault {
                word: s("f"),
                argument: s("c")
            },
            s("c = { power = 2, name = \"a\" }")
        )]
    );
}

/// A default reading the callee's own parameter never matches a caller's
/// parameter of the same name.
#[test]
fn a_default_reading_an_unbound_parameter_is_no_callers_parameter() {
    let source = "template echo(x: Nat, y: Nat = $x): Nat { body = $y }\n\
                  template t(x: Nat): Nat { body = echo(y = $x) }";
    assert_eq!(
        kinds(source),
        [RefusalKind::MissingArgument {
            word: s("echo"),
            argument: s("x")
        }]
    );
}

#[test]
fn a_coercion_composes_with_option() {
    let source = format!(
        "{COERCIONS}\
         template word: Option(Nominal) {{ body = creature }}\n\
         template int: Option(Amount) {{ body = 2 }}\n\
         template param(t: Subtype): Option(Nominal) {{ body = $t }}\n\
         template omitted: Option(Amount) {{ body = none }}"
    );
    assert_eq!(kinds(&source), []);
}

#[test]
fn a_value_no_coercion_reaches_is_refused() {
    let source = format!(
        "{COERCIONS}\
         template s: Amount {{ body = \"two\" }}\n\
         template t(n: Nat): Amount {{ body = $n }}\n\
         template u: CardType {{ body = goblin }}"
    );
    assert_eq!(
        kinds(&source),
        [
            RefusalKind::StringAt(s("Amount")),
            RefusalKind::ParameterSort {
                name: s("n"),
                sort: s("Nat"),
                expected: s("Amount")
            },
            RefusalKind::UnknownWord {
                word: s("goblin"),
                sort: s("CardType")
            },
        ]
    );
}

#[test]
fn a_coercion_is_one_hop() {
    // `CardType` to `Nominal` to `Condition`: refused at load, and a card
    // type at `Condition` is not coerced twice.
    let source = format!(
        "{COERCIONS}\
         coercion(word: Nominal): Condition {{ body = noun($word) }}\n\
         template t: Condition {{ body = creature }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [
            (
                RefusalKind::ChainedCoercion {
                    from: s("CardType"),
                    via: s("Nominal"),
                    to: s("Condition")
                },
                s("coercion(word: Nominal): Condition { body = noun($word) }")
            ),
            (
                RefusalKind::ChainedCoercion {
                    from: s("Subtype"),
                    via: s("Nominal"),
                    to: s("Condition")
                },
                s("coercion(word: Nominal): Condition { body = noun($word) }")
            ),
            (
                RefusalKind::UnknownWord {
                    word: s("creature"),
                    sort: s("Condition")
                },
                s("creature")
            ),
        ]
    );
}

#[test]
fn a_duplicate_coercion_is_refused() {
    let source = format!(
        "{COERCIONS}\
         primitive count(value: Int): Amount = Count\n\
         coercion(n: Int): Amount {{ body = count($n) }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [(
            RefusalKind::DuplicateCoercion {
                from: s("Int"),
                to: s("Amount")
            },
            s("coercion(n: Int): Amount { body = count($n) }")
        )]
    );
}

#[test]
fn a_word_at_two_from_sorts_of_one_target_is_refused_with_no_call_site() {
    let source = format!("{COERCIONS}subtype creature {{ types = [creature] }}");
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [(
            RefusalKind::AmbiguousWord {
                word: s("creature"),
                sorts: vec![s("CardType"), s("Subtype")],
                target: s("Nominal")
            },
            s("subtype creature { types = [creature] }")
        )]
    );
}

#[test]
fn a_word_at_a_target_and_at_a_sort_coerced_into_it_is_refused_with_no_call_site() {
    // A leftover head-noun Template beside its card type.
    let source = format!("{COERCIONS}template creature: Nominal {{ body = type(creature) }}");
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [(
            RefusalKind::AmbiguousWord {
                word: s("creature"),
                sorts: vec![s("CardType"), s("Nominal")],
                target: s("Nominal")
            },
            s("template creature: Nominal { body = type(creature) }")
        )]
    );
}

#[test]
fn an_integer_literal_reaching_a_target_from_int_and_nat_is_refused_with_no_call_site() {
    let source = format!(
        "{COERCIONS}\
         primitive count(value: Nat): Amount = Count\n\
         coercion(n: Nat): Amount {{ body = count($n) }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [(
            RefusalKind::AmbiguousLiteral {
                literal: s("an integer"),
                sorts: vec![s("Int"), s("Nat")],
                target: s("Amount")
            },
            s("coercion(n: Nat): Amount { body = count($n) }")
        )]
    );
}

#[test]
fn a_record_literal_reaching_a_target_from_two_record_sorts_is_refused_with_no_call_site() {
    let source = "primitive ordinalOf(o: EventOrdinal): Condition = OrdinalOf\n\
         primitive faceDownOf(f: FaceDown): Condition = FaceDownOf\n\
         coercion(o: EventOrdinal): Condition { body = ordinalOf($o) }\n\
         coercion(f: FaceDown): Condition { body = faceDownOf($f) }";
    assert_eq!(
        refusals(&[("a.syl", source)]),
        [(
            RefusalKind::AmbiguousLiteral {
                literal: s("a record"),
                sorts: vec![s("EventOrdinal"), s("FaceDown")],
                target: s("Condition")
            },
            s("coercion(f: FaceDown): Condition { body = faceDownOf($f) }")
        )]
    );
}

#[test]
fn a_word_at_two_sorts_no_coercion_joins_is_no_ambiguity() {
    // `snow` is a supertype and a mana symbol; neither reaches a coercion's
    // target.
    let source = format!(
        "{COERCIONS}\
         supertype snow {{ name = \"Snow\" }}\n\
         primitive snowPip: ManaSymbol = Snow\n\
         template snow: ManaSymbol {{ body = snowPip }}"
    );
    assert_eq!(kinds(&source), []);
}

#[test]
fn a_malformed_coercion_is_refused() {
    let source = format!(
        "{COERCIONS}\
         template article: Nominal {{ body = type(creature) }}\n\
         coercion(a: Int, b: Int): Amount {{ body = lit($a) }}\n\
         coercion(a: Int = 1): Amount {{ body = lit($a) }}\n\
         coercion(a: CardType): Nominal {{ body = article }}\n\
         template articleOf(t: CardType): Nominal {{ body = type($t) }}\n\
         coercion(a: CardType): Nominal {{ body = articleOf($a) }}\n\
         coercion(a: Nat): Amount {{ body = lit(1) }}\n\
         coercion named(a: Int): Amount {{ body = lit($a) }}\n\
         coercion(a: Int): Amount\n\
         coercion [1]\n\
         coercion(a: Int): Amount {{}}"
    );
    assert_eq!(
        kinds(&source),
        [
            RefusalKind::WrittenName(s("coercion")),
            RefusalKind::MissingBlock(s("coercion")),
            RefusalKind::MissingHeader(s("coercion")),
            RefusalKind::CoercionParameters,
            RefusalKind::CoercionParameters,
            RefusalKind::CoercionBody(s("Nominal")),
            RefusalKind::CoercionBody(s("Nominal")),
            RefusalKind::CoercionBody(s("Amount")),
            RefusalKind::MissingField {
                field: s("body"),
                at: s("`coercion`(Int): Amount")
            },
        ]
    );
}

#[test]
fn a_card_written_as_a_header_without_a_name_is_refused() {
    assert_eq!(
        kinds("card(a: Int): Card { body = 1 }"),
        [RefusalKind::WrittenHeader(s("card"))]
    );
}

#[test]
fn a_word_at_an_option_of_a_target_and_at_a_from_sort_is_refused_with_no_call_site() {
    // At an `Option(Nominal)` position the `Option` declaration would be
    // found before the coercion from `CardType` is tried.
    let source =
        format!("{COERCIONS}template creature: Option(Nominal) {{ body = type(creature) }}");
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [(
            RefusalKind::AmbiguousWord {
                word: s("creature"),
                sorts: vec![s("CardType"), s("Option(Nominal)")],
                target: s("Option(Nominal)")
            },
            s("template creature: Option(Nominal) { body = type(creature) }")
        )]
    );
}

#[test]
fn a_list_literal_reaching_a_target_from_two_list_sorts_is_refused_with_no_call_site() {
    let source = format!(
        "{COERCIONS}\
         primitive ofTypes(types: List(CardType)): Condition = OfTypes\n\
         primitive ofSubtypes(subtypes: List(Subtype)): Condition = OfSubtypes\n\
         coercion(t: List(CardType)): Condition {{ body = ofTypes($t) }}\n\
         coercion(t: List(Subtype)): Condition {{ body = ofSubtypes($t) }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [(
            RefusalKind::AmbiguousLiteral {
                literal: s("a list"),
                sorts: vec![s("List(CardType)"), s("List(Subtype)")],
                target: s("Condition")
            },
            s("coercion(t: List(Subtype)): Condition { body = ofSubtypes($t) }")
        )]
    );
}

#[test]
fn a_coercion_from_a_sort_to_itself_is_refused() {
    let source = format!(
        "{COERCIONS}\
         primitive twice(n: Amount): Amount = Twice\n\
         coercion(n: Amount): Amount {{ body = twice($n) }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [
            (
                RefusalKind::CoercionToItself(s("Amount")),
                s("coercion(n: Amount): Amount { body = twice($n) }")
            ),
            (
                RefusalKind::ChainedCoercion {
                    from: s("Int"),
                    via: s("Amount"),
                    to: s("Amount")
                },
                s("coercion(n: Amount): Amount { body = twice($n) }")
            ),
        ]
    );
}

#[test]
fn a_nameless_header_on_a_named_kind_is_refused() {
    assert_eq!(
        kinds("template(x: Int): Int { body = $x }"),
        [RefusalKind::MissingName(s("template"))]
    );
}

#[test]
fn a_move_names_no_observation_point() {
    let written = |action: &str| format!("template t: VerbPhrase {{ body = {action} }}");
    // Whether a trigger looks back is derived [CR#603.10a..603.10f]; `move`
    // takes no point to write.
    assert_eq!(
        plugin_refusals(&written("move(it, battlefield, graveyard, [])")),
        []
    );
    assert_eq!(
        plugin_refusals(&written(
            "move(it, battlefield, graveyard, battlefield, [])"
        )),
        [
            (
                RefusalKind::UnknownWord {
                    word: s("battlefield"),
                    sort: s("List(MoveRider)")
                },
                s("battlefield")
            ),
            (RefusalKind::TooManyArguments(s("move")), s("[]")),
        ]
    );
    assert_eq!(
        plugin_refusals(&written(
            "move(it, battlefield, graveyard, [], at = battlefield)"
        )),
        [(
            RefusalKind::UnknownArgument {
                word: s("move"),
                argument: s("at")
            },
            s("at")
        )]
    );
    // The points themselves are no longer words.
    let plugins = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins");
    let builtin = Bundle::load(plugins.join("builtin"), FileId(0)).unwrap();
    for point in ["arrival", "departure"] {
        let source = written(&format!("move(it, battlefield, graveyard, {point}, [])"));
        let canon = bundle("canon", builtin.next_file_id(), &[("cards.syl", &source)]);
        let Err(CheckError::Declarations(_)) = check(&builtin, &[&canon]) else {
            panic!("`{point}` names no declaration");
        };
    }
}

#[test]
fn valid_spellings_and_unspelled_declarations_pass_and_invalid_spellings_are_refused() {
    let valid = format!(
        "{COLORS}\
         primitive unspelled: Color = Unspelled\n\
         primitive gap: Color = Gap {{ spelling = \"^\" }}\n\
         primitive ownName: NounPhrase = OwnName {{ spelling = \"#\" }}\n\
         template shortenedName: NounPhrase {{ spelling = \"##\", body = ownName }}\n\
         primitive see(who: NounPhrase): Color = See\n\
         template tied(p: Color): Color {{ spelling = \"non~$p\", body = $p }}\n\
         template ord(n: Nat): Nat {{ spelling = \"$n@Ordinal\", body = $n }}\n\
         template opt(a: Color, b: Option(Color) = none): Color {{\n\
           spelling = \"pick $a< or $b> %VerbPhrase: for $a\",\n\
           body = $a\n\
         }}"
    );
    assert_eq!(kinds(&valid), []);

    let invalid = format!(
        "{COLORS}\
         template untied(p: Color): Color {{ spelling = \"non$p\", body = $p }}\n\
         template badForm(n: Nat): Nat {{ spelling = \"$n@Plural\", body = $n }}\n\
         template badGap: Color {{ spelling = \"cast ^\", body = white }}\n\
         template badHash: Color {{ spelling = \"cast #\", body = white }}\n\
         template notString: Color {{ spelling = 1, body = white }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &invalid)]),
        [
            (
                RefusalKind::InvalidSpelling(sylex_spelling::ErrorKind::AbuttingHole),
                s("\"non$p\"")
            ),
            (
                RefusalKind::InvalidSpelling(sylex_spelling::ErrorKind::UnknownForm(s("Plural"))),
                s("\"$n@Plural\"")
            ),
            (
                RefusalKind::InvalidSpelling(sylex_spelling::ErrorKind::GapNotWholeSpelling),
                s("\"cast ^\"")
            ),
            (
                RefusalKind::InvalidSpelling(sylex_spelling::ErrorKind::UnexpectedMark('#')),
                s("\"cast #\"")
            ),
            (RefusalKind::SpellingNotString, s("1")),
        ]
    );
}

#[test]
fn shared_verb_declarations_and_lexical_verb_references_are_checked() {
    let valid = format!(
        "{COLORS}\
         verb face {{\n\
           lexical = {{lemma = \"face\", frames = [Monotransitive]}}\n\
         }}\n\
         template a: Color {{\n\
           lexical = {{verbs = [face]}},\n\
           body = white\n\
         }}\n\
         template b: Color {{\n\
           lexical = {{verbs = [face]}},\n\
           body = blue\n\
         }}"
    );
    assert_eq!(kinds(&valid), []);

    assert_eq!(
        RefusalKind::InvalidSharedVerb(s("empty")).to_string(),
        "shared verb `empty` needs a `lexical` record with a non-empty `lemma` and `frames`"
    );

    let invalid = format!(
        "{COLORS}\
         verb empty {{}}\n\
         verb face {{\n\
           lexical = {{lemma = \"face\", frames = [Monotransitive]}}\n\
         }}\n\
         primitive fakeVerb: Verb = FakeVerb\n\
         template nonRecordLexical: Color {{\n\
           lexical = \"bad\",\n\
           body = white\n\
         }}\n\
         template nonListVerbs: Color {{\n\
           lexical = {{verbs = \"bad\"}},\n\
           body = white\n\
         }}\n\
         template badInlineVerb: Color {{\n\
           lexical = {{verbs = [{{lemma = 1, frames = [Monotransitive]}}]}},\n\
           body = white\n\
         }}\n\
         template wrongSort: Color {{\n\
           lexical = {{verbs = [white]}},\n\
           body = white\n\
         }}\n\
         template duplicateInline: Color {{\n\
           lexical = {{verbs = [{{lemma = \"face\", frames = [Monotransitive]}}]}},\n\
           body = blue\n\
         }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &invalid)]),
        [
            (
                RefusalKind::ReservedConstructor(s("Verb.FakeVerb")),
                s("FakeVerb")
            ),
            (
                RefusalKind::InvalidSharedVerb(s("empty")),
                s("verb empty {}")
            ),
            (RefusalKind::LexicalNotRecord, s("\"bad\"")),
            (RefusalKind::LexicalVerbsNotList, s("\"bad\"")),
            (
                RefusalKind::InvalidLexicalVerb,
                s("{lemma = 1, frames = [Monotransitive]}")
            ),
            (
                RefusalKind::UnknownWord {
                    word: s("white"),
                    sort: s("Verb")
                },
                s("white")
            ),
            (
                RefusalKind::DuplicateSharedVerb {
                    lemma: s("face"),
                    verb: s("face")
                },
                s("\"face\"")
            ),
        ]
    );
}

const ACTOR_PREAMBLE: &str = "\
    primitive you: NounPhrase = You\n\
    primitive see(who: NounPhrase, opt: Option(NounPhrase) = none): VerbPhrase = See\n\
    primitive count(n: Nat): VerbPhrase = Count\n\
    primitive spell(action: VerbPhrase): Ability = Spell\n\
    primitive single(face: Characteristics): Card = Single\n";

#[test]
fn an_action_definition_resolves_its_implicit_actor_parameter() {
    let valid = format!(
        "{ACTOR_PREAMBLE}\
         action act {{ body = see($actor, opt = $actor) }}\n\
         template callAct: VerbPhrase {{ body = act }}"
    );
    assert_eq!(kinds(&valid), []);

    let invalid = format!(
        "{ACTOR_PREAMBLE}\
         action badSort {{ body = count($actor) }}\n\
         action badDefault(who: NounPhrase = $actor) {{ body = see($who) }}\n\
         action redeclared(actor: NounPhrase) {{ body = see($actor) }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &invalid)]),
        [
            (RefusalKind::DuplicateParameter(s("actor")), s("actor")),
            (
                RefusalKind::ParameterSort {
                    name: s("actor"),
                    sort: s("NounPhrase"),
                    expected: s("Nat"),
                },
                s("$actor"),
            ),
            (RefusalKind::UnknownParameter(s("actor")), s("$actor")),
        ]
    );
}

#[test]
fn actor_parameter_in_a_template_is_refused() {
    let source = format!(
        "{ACTOR_PREAMBLE}\
         template t: VerbPhrase {{ body = see($actor) }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [(RefusalKind::UnknownParameter(s("actor")), s("$actor"))]
    );
}

#[test]
fn actor_parameter_in_another_declaration_kind_is_refused() {
    let source = format!(
        "{ACTOR_PREAMBLE}\
         primitive yes: Nominal = Yes\n\
         primitive always: Condition = Always\n\
         ability kw {{ body = [spell(see($actor))] }}\n\
         ability_word aw {{ body = spell(see($actor)) }}\n\
         sba rule {{ body = {{ scope = yes, when = always, then = see($actor) }} }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [
            (RefusalKind::UnknownParameter(s("actor")), s("$actor")),
            (RefusalKind::UnknownParameter(s("actor")), s("$actor")),
            (RefusalKind::UnknownParameter(s("actor")), s("$actor")),
        ]
    );
}

#[test]
fn actor_parameter_in_a_card_is_refused() {
    let source = format!(
        "{ACTOR_PREAMBLE}\
         card single({{ text = [spell(see($actor))] }})"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [(RefusalKind::UnknownParameter(s("actor")), s("$actor"))]
    );
}

#[test]
fn spelling_rule_1_hole_names_unknown_parameter() {
    let source = format!(
        "{ACTOR_PREAMBLE}\
         template t(a: Nat = 1): Nat {{ spelling = \"draw $b\", body = $a }}\n\
         action act {{ spelling = \"$actor acts\", body = see($actor) }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [
            (RefusalKind::SpellingUnknownHole(s("b")), s("\"draw $b\"")),
            (
                RefusalKind::SpellingUnknownHole(s("actor")),
                s("\"$actor acts\"")
            ),
        ]
    );
}

#[test]
fn spelling_rule_2_join_matches_list_sort() {
    let source = format!(
        "{COLORS}\
         template okList(cs: List(Color)): List(Color) {{ spelling = \"$cs#<and>\", body = $cs }}\n\
         template okOptList(cs: Option(List(Color)) = none): Option(List(Color)) {{\n\
           spelling = \"colors< $cs#<or>>\",\n\
           body = $cs\n\
         }}\n\
         template joinOnScalar(c: Color): Color {{ spelling = \"$c#<and>\", body = $c }}\n\
         template listNoJoin(cs: List(Color)): List(Color) {{ spelling = \"$cs\", body = $cs }}\n\
         template optListNoJoin(cs: Option(List(Color)) = none): Option(List(Color)) {{\n\
           spelling = \"colors< $cs>\",\n\
           body = $cs\n\
         }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [
            (
                RefusalKind::SpellingJoinNotList {
                    name: s("c"),
                    sort: s("Color"),
                },
                s("\"$c#<and>\"")
            ),
            (
                RefusalKind::SpellingListWithoutJoin {
                    name: s("cs"),
                    sort: s("List(Color)"),
                },
                s("\"$cs\"")
            ),
            (
                RefusalKind::SpellingListWithoutJoin {
                    name: s("cs"),
                    sort: s("Option(List(Color))"),
                },
                s("\"colors< $cs>\"")
            ),
        ]
    );
}

#[test]
fn spelling_rule_3_hole_in_optional_requires_omissible_parameter() {
    let source = format!(
        "{COLORS}\
         template okDefault(c: Color = white): Color {{ spelling = \"pick< $c>\", body = $c }}\n\
         template okOption(c: Option(Color)): Option(Color) {{ spelling = \"pick< $c>\", body = $c }}\n\
         template okList(cs: List(Color)): List(Color) {{ spelling = \"pick< $cs#<and>>\", body = $cs }}\n\
         template badRequired(c: Color): Color {{ spelling = \"pick< $c>\", body = $c }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [(
            RefusalKind::SpellingOptionalInomissible {
                name: s("c"),
                sort: s("Color"),
            },
            s("\"pick< $c>\"")
        )]
    );
}

#[test]
fn spelling_rule_4_holes_of_one_parameter_share_one_scope() {
    let source = format!(
        "{COLORS}\
         template okOutside(c: Color): Color {{ spelling = \"$c and $c\", body = $c }}\n\
         template okSameOpt(c: Option(Color)): Option(Color) {{\n\
           spelling = \"pick< $c or $c>\",\n\
           body = $c\n\
         }}\n\
         template splitOutsideAndInside(c: Option(Color)): Option(Color) {{\n\
           spelling = \"$c< or $c>\",\n\
           body = $c\n\
         }}\n\
         template splitTwoOpts(c: Option(Color)): Option(Color) {{\n\
           spelling = \"pick< first $c>< then $c>\",\n\
           body = $c\n\
         }}\n\
         template splitNestedOpt(a: Option(Color), b: Option(Color)): Option(Color) {{\n\
           spelling = \"pick< $a< and $b with $a>>\",\n\
           body = $a\n\
         }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [
            (
                RefusalKind::SpellingSplitParameter(s("c")),
                s("\"$c< or $c>\"")
            ),
            (
                RefusalKind::SpellingSplitParameter(s("c")),
                s("\"pick< first $c>< then $c>\"")
            ),
            (
                RefusalKind::SpellingSplitParameter(s("a")),
                s("\"pick< $a< and $b with $a>>\"")
            ),
        ]
    );
}

#[test]
fn spelling_rule_5_optional_part_must_directly_hold_a_hole() {
    let source = format!(
        "{COLORS}\
         template okTailHole(c: Option(Color)): Option(Color) {{\n\
           spelling = \"pick< %VerbPhrase: for $c>\",\n\
           body = $c\n\
         }}\n\
         template fixedOnly: Color {{ spelling = \"white< mana>\", body = white }}\n\
         template onlyNested(c: Option(Color)): Option(Color) {{\n\
           spelling = \"pick< mana< of $c>>\",\n\
           body = $c\n\
         }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [
            (
                RefusalKind::SpellingOptionalWithoutHole,
                s("\"white< mana>\"")
            ),
            (
                RefusalKind::SpellingOptionalWithoutHole,
                s("\"pick< mana< of $c>>\"")
            ),
        ]
    );
}

#[test]
fn spelling_rule_6_something_outside_optionals_is_certain_to_print() {
    let source = format!(
        "{COLORS}\
         template okTailOnly(c: Color): Color {{ spelling = \"%VerbPhrase: for $c\", body = $c }}\n\
         template emptySpelling: Color {{ spelling = \"\", body = white }}\n\
         template allOptional(a: Option(Color)): Option(Color) {{\n\
           spelling = \"<from $a>\",\n\
           body = $a\n\
         }}\n\
         template onlySpaceBetweenOptionals(a: Option(Color), b: Option(Color)): Option(Color) {{\n\
           spelling = \"<from $a> <until $b>\",\n\
           body = $a\n\
         }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [
            (RefusalKind::SpellingNothingCertain, s("\"\"")),
            (RefusalKind::SpellingNothingCertain, s("\"<from $a>\"")),
            (
                RefusalKind::SpellingNothingCertain,
                s("\"<from $a> <until $b>\"")
            ),
        ]
    );
}

#[test]
fn spelling_rule_7_unprinted_parameter_must_be_omissible() {
    let source = format!(
        "{COLORS}\
         template okDefault(c: Color = white): Color {{ spelling = \"colored\", body = $c }}\n\
         template okOption(c: Option(Color)): Option(Color) {{ spelling = \"colored\", body = $c }}\n\
         template okList(cs: List(Color)): List(Color) {{ spelling = \"colored\", body = $cs }}\n\
         template badUnprinted(c: Color): Color {{ spelling = \"colored\", body = $c }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [(
            RefusalKind::SpellingUnprintedParameter {
                name: s("c"),
                sort: s("Color"),
            },
            s("\"colored\"")
        )]
    );
}

#[test]
fn spelling_rule_8_skeleton_must_be_unambiguous() {
    let source = format!(
        "{COLORS}\
         primitive lit(n: Int): Amount = Lit\n\
         coercion(n: Int): Amount {{ body = lit($n) }}\n\
         template okDistinctWords(start: Option(Color), end: Option(Color)): Option(Color) {{\n\
           spelling = \"<from $start >until< $end>\",\n\
           body = $start\n\
         }}\n\
         template okDifferentSorts(c: Option(Color), n: Option(Nat)): Option(Color) {{\n\
           spelling = \"pick< $c>< $n>\",\n\
           body = $c\n\
         }}\n\
         template okTailSeparates(a: Color, b: Option(Color), c: Option(Color)): Color {{\n\
           spelling = \"$a< $b> %VerbPhrase: <$c>\",\n\
           body = $a\n\
         }}\n\
         template twoBareOptionals(a: Option(Color), b: Option(Color)): Option(Color) {{\n\
           spelling = \"pick< $a>< $b>\",\n\
           body = $a\n\
         }}\n\
         template coercedOptionals(a: Option(Int), b: Option(Amount)): Option(Amount) {{\n\
           spelling = \"count< $a>< $b>\",\n\
           body = $b\n\
         }}\n\
         template listAndOptionalSameJoin(cs: List(Color), extra: Option(Color)): List(Color) {{\n\
           spelling = \"$cs#<and>< and $extra>\",\n\
           body = $cs\n\
         }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [
            (
                RefusalKind::SpellingAmbiguousSkeleton,
                s("\"pick< $a>< $b>\"")
            ),
            (
                RefusalKind::SpellingAmbiguousSkeleton,
                s("\"count< $a>< $b>\"")
            ),
            (
                RefusalKind::SpellingAmbiguousSkeleton,
                s("\"$cs#<and>< and $extra>\"")
            ),
        ]
    );
}

#[test]
fn spelling_tie_affix_must_be_a_declared_bound_form() {
    let source = format!(
        "{COLORS}\
         template okNon(c: Color): Color {{ spelling = \"non~$c\", body = $c }}\n\
         template okWalk(c: Color): Color {{ spelling = \"$c~walk\", body = $c }}\n\
         template okCycling(c: Color): Color {{ spelling = \"$c~cycling\", body = $c }}\n\
         template badPrefix(c: Color): Color {{ spelling = \"pre~$c\", body = $c }}\n\
         template badSuffix(c: Color): Color {{ spelling = \"$c~able\", body = $c }}\n\
         template wrongSidePrefix(c: Color): Color {{ spelling = \"walk~$c\", body = $c }}\n\
         template wrongSideSuffix(c: Color): Color {{ spelling = \"$c~non\", body = $c }}\n\
         template stemIsNotAffix(c: Color): Color {{ spelling = \"island~$c\", body = $c }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [
            (
                RefusalKind::SpellingUnknownAffix {
                    affix: s("pre"),
                    side: "prefix",
                },
                s("\"pre~$c\"")
            ),
            (
                RefusalKind::SpellingUnknownAffix {
                    affix: s("able"),
                    side: "suffix",
                },
                s("\"$c~able\"")
            ),
            (
                RefusalKind::SpellingUnknownAffix {
                    affix: s("walk"),
                    side: "prefix",
                },
                s("\"walk~$c\"")
            ),
            (
                RefusalKind::SpellingUnknownAffix {
                    affix: s("non"),
                    side: "suffix",
                },
                s("\"$c~non\"")
            ),
            (
                RefusalKind::SpellingUnknownAffix {
                    affix: s("island"),
                    side: "prefix",
                },
                s("\"island~$c\"")
            ),
        ]
    );
}
#[test]
fn triggered_refuses_a_bare_when_whenever_or_at_condition() {
    assert_eq!(
        RefusalKind::SingleConditionTriggered(s("when")).to_string(),
        "`triggered` over a single `when` condition; write `when` at Ability"
    );
    assert_eq!(
        plugin_refusals(
            "template a: Ability {\n\
               body = triggered(when(does(actor(thisCreature), enter)), draw(1))\n\
             }\n\
             template b: Ability {\n\
               body = triggered(\n\
                 whenever(does(actor(thisCreature), attack)),\n\
                 intervening = is(you, hasDesignation(monarch)),\n\
                 draw(1))\n\
             }\n\
             template c: Ability {\n\
               body = triggered(at(beginningOfYour(upkeep)), draw(1))\n\
             }"
        ),
        [
            (
                RefusalKind::SingleConditionTriggered(s("when")),
                s("triggered(when(does(actor(thisCreature), enter)), draw(1))")
            ),
            (
                RefusalKind::SingleConditionTriggered(s("whenever")),
                s("triggered(\n\
                 whenever(does(actor(thisCreature), attack)),\n\
                 intervening = is(you, hasDesignation(monarch)),\n\
                 draw(1))")
            ),
            (
                RefusalKind::SingleConditionTriggered(s("at")),
                s("triggered(at(beginningOfYour(upkeep)), draw(1))")
            ),
        ]
    );

    // Only `(w, Ability)`'s own root body may write `triggered(w(...), ...)`.
    let synthetic = "\
        primitive ev: GameEvent = Ev\n\
        primitive pt: TurnPoint = Pt\n\
        primitive act: VerbPhrase = Act\n\
        primitive triggerEvent(e: GameEvent): TriggerCondition = Event\n\
        primitive reached(p: TurnPoint): GameEvent = Reached\n\
        primitive triggered(c: TriggerCondition, a: VerbPhrase): Ability = Triggered\n\
        template when(e: GameEvent): TriggerCondition { body = triggerEvent($e) }\n\
        template whenever(e: GameEvent): TriggerCondition { body = triggerEvent($e) }\n\
        template at(p: TurnPoint): TriggerCondition { body = triggerEvent(reached($p)) }\n\
        template when(e: GameEvent, a: VerbPhrase): Ability { body = triggered(when($e), $a) }\n\
        template at(p: TurnPoint, a: VerbPhrase): Ability { body = triggered(at($p), $a) }\n\
        template whenever(e: GameEvent, a: VerbPhrase): Ability {\n\
          body = triggered(when($e), $a)\n\
        }\n\
        template alias(e: GameEvent, a: VerbPhrase): Ability {\n\
          body = triggered(when($e), $a)\n\
        }";
    assert_eq!(
        refusals(&[("a.syl", synthetic)]),
        [
            (
                RefusalKind::SingleConditionTriggered(s("when")),
                s("triggered(when($e), $a)")
            ),
            (
                RefusalKind::SingleConditionTriggered(s("when")),
                s("triggered(when($e), $a)")
            ),
        ]
    );
}

#[test]
fn trigger_condition_and_and_or_refuse_fewer_than_two_conditions() {
    assert_eq!(
        RefusalKind::EmptyCoordination {
            word: s("or"),
            sort: s("TriggerCondition"),
        }
        .to_string(),
        "empty `or` at TriggerCondition; write `or` with two or more items"
    );
    assert_eq!(
        RefusalKind::SingleItemCoordination {
            word: s("and"),
            sort: s("TriggerCondition"),
            instead: s("when(...)"),
        }
        .to_string(),
        "`and` at TriggerCondition has one item; write `when(...)` directly"
    );
    assert_eq!(
        plugin_refusals(
            "template a: Ability { body = triggered(or([]), draw(1)) }\n\
             template b: Ability { body = triggered(and([]), draw(1)) }\n\
             template c: Ability {\n\
               body = triggered(or([when(does(actor(thisCreature), enter))]), draw(1))\n\
             }\n\
             template d: Ability {\n\
               body = triggered(and([at(beginningOfYour(upkeep))]), draw(1))\n\
             }"
        ),
        [
            (
                RefusalKind::EmptyCoordination {
                    word: s("or"),
                    sort: s("TriggerCondition"),
                },
                s("or([])")
            ),
            (
                RefusalKind::EmptyCoordination {
                    word: s("and"),
                    sort: s("TriggerCondition"),
                },
                s("and([])")
            ),
            (
                RefusalKind::SingleItemCoordination {
                    word: s("or"),
                    sort: s("TriggerCondition"),
                    instead: s("when(...)"),
                },
                s("or([when(does(actor(thisCreature), enter))])")
            ),
            (
                RefusalKind::SingleItemCoordination {
                    word: s("and"),
                    sort: s("TriggerCondition"),
                    instead: s("at(...)"),
                },
                s("and([at(beginningOfYour(upkeep))])")
            ),
        ]
    );
}

#[test]
fn trigger_condition_and_and_or_refuse_direct_nesting() {
    assert_eq!(
        RefusalKind::NestedTriggerCondition {
            outer: s("and"),
            inner: s("or"),
        }
        .to_string(),
        "nested `or` inside `and` at TriggerCondition; write one flat `and([...])` list"
    );
    assert_eq!(
        plugin_refusals(
            "template a(x: TriggerCondition, y: TriggerCondition, z: TriggerCondition): TriggerCondition {\n\
               body = and([$x, and([$y, $z])])\n\
             }\n\
             template b(x: TriggerCondition, y: TriggerCondition, z: TriggerCondition): TriggerCondition {\n\
               body = and([$x, or([$y, $z])])\n\
             }\n\
             template c(x: TriggerCondition, y: TriggerCondition, z: TriggerCondition): TriggerCondition {\n\
               body = or([$x, and([$y, $z])])\n\
             }\n\
             template d(x: TriggerCondition, y: TriggerCondition, z: TriggerCondition): TriggerCondition {\n\
               body = or([$x, or([$y, $z])])\n\
             }"
        ),
        [
            (
                RefusalKind::NestedTriggerCondition {
                    outer: s("and"),
                    inner: s("and"),
                },
                s("and([$y, $z])")
            ),
            (
                RefusalKind::NestedTriggerCondition {
                    outer: s("and"),
                    inner: s("or"),
                },
                s("or([$y, $z])")
            ),
            (
                RefusalKind::NestedTriggerCondition {
                    outer: s("or"),
                    inner: s("and"),
                },
                s("and([$y, $z])")
            ),
            (
                RefusalKind::NestedTriggerCondition {
                    outer: s("or"),
                    inner: s("or"),
                },
                s("or([$y, $z])")
            ),
        ]
    );
}

#[test]
fn conjunction_and_disjunction_primitives_refuse_fewer_than_two_items() {
    assert_eq!(
        plugin_refusals(
            "template ev0: GameEvent { body = or([]) }\n\
             template ev1: GameEvent { body = or([does(actor(you), shuffle)]) }\n\
             template pm0: ProducedMana { body = or([]) }\n\
             template pm1: ProducedMana { body = or([symbols([colorlessPip])]) }\n\
             template cond0: Condition { body = and([]) }\n\
             template cond1(c: Condition): Condition { body = or([$c]) }\n\
             template cost0: Cost { body = compound([]) }\n\
             template cost1: Cost { body = compound([tapSymbol]) }\n\
             template costOr1: Cost { body = or([tapSymbol]) }\n\
             template spec1: StaticSpec { body = conjunction([getsPt(it, up(1), up(1))]) }\n\
             template tim0: Timing { body = and([]) }\n\
             template tim1: Timing { body = or([anyTiming]) }\n\
             template deck0: DeckTrait { body = anyOf([]) }"
        ),
        [
            (
                RefusalKind::EmptyCoordination {
                    word: s("or"),
                    sort: s("GameEvent"),
                },
                s("or([])")
            ),
            (
                RefusalKind::SingleItemCoordination {
                    word: s("or"),
                    sort: s("GameEvent"),
                    instead: s("does(...)"),
                },
                s("or([does(actor(you), shuffle)])")
            ),
            (
                RefusalKind::EmptyCoordination {
                    word: s("or"),
                    sort: s("ProducedMana"),
                },
                s("or([])")
            ),
            (
                RefusalKind::SingleItemCoordination {
                    word: s("or"),
                    sort: s("ProducedMana"),
                    instead: s("symbols(...)"),
                },
                s("or([symbols([colorlessPip])])")
            ),
            (
                RefusalKind::EmptyCoordination {
                    word: s("and"),
                    sort: s("Condition"),
                },
                s("and([])")
            ),
            (
                RefusalKind::SingleItemCoordination {
                    word: s("or"),
                    sort: s("Condition"),
                    instead: s("$c"),
                },
                s("or([$c])")
            ),
            (
                RefusalKind::EmptyCoordination {
                    word: s("compound"),
                    sort: s("Cost"),
                },
                s("compound([])")
            ),
            (
                RefusalKind::SingleItemCoordination {
                    word: s("compound"),
                    sort: s("Cost"),
                    instead: s("tapSymbol"),
                },
                s("compound([tapSymbol])")
            ),
            (
                RefusalKind::SingleItemCoordination {
                    word: s("or"),
                    sort: s("Cost"),
                    instead: s("tapSymbol"),
                },
                s("or([tapSymbol])")
            ),
            (
                RefusalKind::SingleItemCoordination {
                    word: s("conjunction"),
                    sort: s("StaticSpec"),
                    instead: s("getsPt(...)"),
                },
                s("conjunction([getsPt(it, up(1), up(1))])")
            ),
            (
                RefusalKind::EmptyCoordination {
                    word: s("and"),
                    sort: s("Timing"),
                },
                s("and([])")
            ),
            (
                RefusalKind::SingleItemCoordination {
                    word: s("or"),
                    sort: s("Timing"),
                    instead: s("anyTiming"),
                },
                s("or([anyTiming])")
            ),
            (
                RefusalKind::EmptyCoordination {
                    word: s("anyOf"),
                    sort: s("DeckTrait"),
                },
                s("anyOf([])")
            ),
        ]
    );
}

#[test]
fn coordination_templates_refuse_fewer_than_two_items() {
    // Non-TriggerCondition cross-operator nesting stays valid.
    assert_eq!(
        plugin_refusals(
            "template okPred: Predicate {\n\
               body = and([hasType(creature), or([hasColor(white), hasColor(blue)])])\n\
             }\n\
             template okCond(a: Condition, b: Condition, c: Condition): Condition {\n\
               body = and([$a, or([$b, $c])])\n\
             }"
        ),
        []
    );

    assert_eq!(
        plugin_refusals(
            "template pred0: Predicate { body = and([]) }\n\
             template nom1: Nominal { body = andOr([creature]) }\n\
             template np1: NounPhrase { body = or([you]) }\n\
             template loc1: Location { body = and([hand]) }\n\
             template est1: VerbPhrase { body = establishFor(it, [getsPt(it, up(1), up(1))]) }"
        ),
        [
            (
                RefusalKind::EmptyCoordination {
                    word: s("and"),
                    sort: s("Predicate"),
                },
                s("and([])")
            ),
            (
                RefusalKind::SingleItemCoordination {
                    word: s("andOr"),
                    sort: s("Nominal"),
                    instead: s("creature"),
                },
                s("andOr([creature])")
            ),
            (
                RefusalKind::SingleItemCoordination {
                    word: s("or"),
                    sort: s("NounPhrase"),
                    instead: s("you"),
                },
                s("or([you])")
            ),
            (
                RefusalKind::SingleItemCoordination {
                    word: s("and"),
                    sort: s("Location"),
                    instead: s("hand"),
                },
                s("and([hand])")
            ),
            (
                RefusalKind::SingleItemCoordination {
                    word: s("establishFor"),
                    sort: s("VerbPhrase"),
                    instead: s("establish(...)"),
                },
                s("establishFor(it, [getsPt(it, up(1), up(1))])")
            ),
        ]
    );
}

const NOMINAL_PREAMBLE: &str = "\
    primitive and: Coordinator = And\n\
    primitive or: Coordinator = Or\n\
    primitive white: PreModifier = White\n\
    primitive black: PreModifier = Black\n\
    primitive tapped: PreModifier = Tapped\n\
    primitive youControl: PostModifier = YouControl\n\
    primitive card: Nominal = Card\n\
    primitive creature: Nominal = Creature\n\
    primitive zombie: Nominal = Zombie\n\
    primitive nominal(pre: List(PreModifier) = [], head: Nominal, post: List(PostModifier) = []): Nominal = Nominal\n\
    primitive coord(coordinator: Coordinator, items: List(Nominal)): Nominal = Coordination\n\
    template or(disjuncts: List(Nominal)): Nominal { body = coord(or, $disjuncts) }\n\
    primitive coord(coordinator: Coordinator, items: List(PreModifier)): PreModifier = Coordination\n\
    template or(disjuncts: List(PreModifier)): PreModifier { body = coord(or, $disjuncts) }\n\
    template andOr(disjuncts: List(PreModifier)): PreModifier { body = or($disjuncts) }\n\
    template stacked(modifiers: List(PreModifier)): PreModifier { body = coord(and, $modifiers) }\n\
    primitive non(p: PreModifier): PreModifier = Non\n";

#[test]
fn nominal_allows_coordinated_heads_and_refuses_bare_and_nested_nominals() {
    let valid = format!(
        "{NOMINAL_PREAMBLE}\
         template preOnly: Nominal {{ body = nominal(pre = [white], creature) }}\n\
         template postOnly: Nominal {{ body = nominal(creature, post = [youControl]) }}\n\
         template both: Nominal {{ body = nominal(pre = [white], creature, post = [youControl]) }}\n\
         template coordHead: Nominal {{\n\
           body = nominal(\n\
             or([nominal(pre = [white], creature), nominal(pre = [black], zombie)]),\n\
             post = [youControl])\n\
         }}\n\
         template withPost(n: Nominal): Nominal {{ body = nominal($n, post = [youControl]) }}\n\
         template headWord: Nominal {{ body = withPost(creature) }}\n\
         template coordArgument: Nominal {{\n\
           body = withPost(or([nominal(pre = [white], creature), zombie]))\n\
         }}"
    );
    assert_eq!(kinds(&valid), []);

    let invalid = format!(
        "{NOMINAL_PREAMBLE}\
         template bare: Nominal {{ body = nominal(creature) }}\n\
         template nestedPre: Nominal {{ body = nominal(pre = [white], nominal(pre = [tapped], creature)) }}\n\
         template nestedPost: Nominal {{ body = nominal(nominal(pre = [white], creature), post = [youControl]) }}\n\
         template whiteCreature: Nominal {{ body = nominal(pre = [white], creature) }}\n\
         template throughBody: Nominal {{ body = nominal(whiteCreature, post = [youControl]) }}\n\
         template withPost(n: Nominal): Nominal {{ body = nominal($n, post = [youControl]) }}\n\
         template throughParameter: Nominal {{ body = withPost(nominal(pre = [black], zombie)) }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &invalid)]),
        [
            (RefusalKind::RedundantNominal, s("nominal(creature)")),
            (
                RefusalKind::NestedNominal,
                s("nominal(pre = [tapped], creature)")
            ),
            (
                RefusalKind::NestedNominal,
                s("nominal(pre = [white], creature)")
            ),
            (RefusalKind::NestedNominal, s("whiteCreature")),
            (
                RefusalKind::NestedNominal,
                s("nominal(pre = [black], zombie)")
            ),
        ]
    );

    // An empty list is its default, refused as such; the nominal rules run
    // only on bundles every earlier rule accepts.
    let empty = format!(
        "{NOMINAL_PREAMBLE}\
         template emptyPre: Nominal {{ body = nominal(pre = [], creature) }}\n\
         template emptyPost: Nominal {{ body = nominal(creature, post = []) }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &empty)]),
        [
            (
                RefusalKind::RedundantDefault {
                    word: s("nominal"),
                    argument: s("pre"),
                },
                s("pre = []")
            ),
            (
                RefusalKind::RedundantDefault {
                    word: s("nominal"),
                    argument: s("post"),
                },
                s("post = []")
            ),
        ]
    );
}

#[test]
fn stacked_requires_two_elements_inside_a_premodifier_coordination() {
    let valid = format!(
        "{NOMINAL_PREAMBLE}\
         template inOr: Nominal {{\n\
           body = nominal(pre = [or([white, stacked([tapped, black])])], card)\n\
         }}\n\
         template inAndOr: Nominal {{\n\
           body = nominal(pre = [andOr([white, stacked([tapped, black])])], card)\n\
         }}\n\
         template inCoord: Nominal {{\n\
           body = nominal(pre = [coord(or, [white, stacked([tapped, black, white])])], card)\n\
         }}\n\
         template whiteBlack: PreModifier {{ body = stacked([white, black]) }}\n\
         template namedArm: Nominal {{ body = nominal(pre = [or([tapped, whiteBlack])], card) }}\n\
         primitive andOr: Coordinator = AndOr\n\
         template inAndOrCoord: Nominal {{\n\
           body = nominal(pre = [coord(andOr, [white, stacked([tapped, black])])], card)\n\
         }}"
    );
    assert_eq!(kinds(&valid), []);

    let invalid = format!(
        "{NOMINAL_PREAMBLE}\
         template directInPre: Nominal {{ body = nominal(pre = [stacked([white, black])], card) }}\n\
         template underNon: Nominal {{ body = nominal(pre = [or([white, non(stacked([tapped, black]))])], card) }}\n\
         template nestedInStacked: Nominal {{ body = nominal(pre = [or([white, stacked([tapped, stacked([black, white])])])], card) }}\n\
         template literalAnd: Nominal {{ body = nominal(pre = [coord(and, [white, black])], card) }}\n\
         template whiteBlack: PreModifier {{ body = stacked([white, black]) }}\n\
         template namedInPre: Nominal {{ body = nominal(pre = [whiteBlack], card) }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &invalid)]),
        [
            (RefusalKind::UnplacedStacked, s("stacked([white, black])")),
            (RefusalKind::UnplacedStacked, s("stacked([tapped, black])")),
            (RefusalKind::UnplacedStacked, s("stacked([black, white])")),
            (
                RefusalKind::UnplacedStacked,
                s("coord(and, [white, black])")
            ),
            (RefusalKind::UnplacedStacked, s("whiteBlack")),
        ]
    );

    let short = format!(
        "{NOMINAL_PREAMBLE}\
         template empty: Nominal {{ body = nominal(pre = [or([white, stacked([])])], card) }}\n\
         template single: Nominal {{ body = nominal(pre = [or([white, stacked([black])])], card) }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &short)]),
        [
            (
                RefusalKind::EmptyCoordination {
                    word: s("stacked"),
                    sort: s("PreModifier"),
                },
                s("stacked([])")
            ),
            (
                RefusalKind::SingleItemCoordination {
                    word: s("stacked"),
                    sort: s("PreModifier"),
                    instead: s("black"),
                },
                s("stacked([black])")
            ),
        ]
    );
}

const AGREEMENT_PREAMBLE: &str = "\
    primitive cardName: QualitySort = CardName\n\
    primitive differ: Agreement = Differ\n\
    primitive agree(axis: QualitySort, agreement: Agreement): PostModifier = Agree\n\
    template withDifferentNames: PostModifier { body = agree(cardName, differ) }\n\
    primitive one: Plurality = One\n\
    primitive many: Plurality = Many\n\
    primitive unmarked: ChoiceMode = Unmarked\n\
    primitive lit(value: Int): Amount = Literal\n\
    coercion(value: Int): Amount { body = lit($value) }\n\
    primitive chosenNumber: Amount = ChosenNumber\n\
    primitive range(low: Option(Nat) = none, high: Option(Nat) = none): Quantity = Range\n\
    primitive exactlyOf(amount: Amount): Quantity = ExactlyOf\n\
    primitive upToOf(amount: Amount): Quantity = UpToOf\n\
    primitive moreThanOf(amount: Amount): Quantity = MoreThanOf\n\
    primitive additional(quantity: Quantity): Quantity = Additional\n\
    primitive theLast: Quantity = Last\n\
    primitive worth(n: Nat): Quantity = Worth\n\
    primitive all: DetPhrase = All\n\
    primitive bare: DetPhrase = Bare\n\
    primitive each: DetPhrase = Each\n\
    primitive a(mode: ChoiceMode): DetPhrase = A\n\
    primitive the(plur: Plurality = one): DetPhrase = The\n\
    primitive count(quantity: Quantity, mode: Option(ChoiceMode)): DetPhrase = Count\n\
    primitive target(quantity: Quantity): DetPhrase = Target\n\
    primitive possessive(owner: NounPhrase): DetPhrase = Possessive\n\
    primitive described(determiner: DetPhrase, nominal: Nominal): NounPhrase = Described\n\
    template a(n: Nominal): NounPhrase { body = described(a(unmarked), $n) }\n\
    template each(n: Nominal): NounPhrase { body = described(each, $n) }\n\
    template the(n: Nominal): NounPhrase { body = described(the, $n) }\n\
    template thePlural(n: Nominal): NounPhrase { body = described(the(plur = many), $n) }\n\
    template allOf(n: Nominal): NounPhrase { body = described(all, $n) }\n\
    template bare(n: Nominal): NounPhrase { body = described(bare, $n) }\n\
    template counted(q: Quantity, n: Nominal): NounPhrase { body = described(count($q, none), $n) }\n\
    template target(n: Nominal): NounPhrase { body = described(target(range(low = 1, high = 1)), $n) }\n\
    template targets(q: Quantity, n: Nominal): NounPhrase { body = described(target($q), $n) }\n\
    primitive whole: SliceCount = Whole\n\
    primitive sliceCounted(quantity: Quantity): SliceCount = Counted\n\
    primitive totaling(bound: Amount): SliceCount = Totaling\n\
    primitive someOf(count: SliceCount, description: Option(Nominal) = none, group: NounPhrase): NounPhrase = SomeOf\n\
    primitive numberOf(group: NounPhrase): Amount = CountOf\n\
    template countOf(n: Nominal): Amount { body = numberOf(described(all, $n)) }\n\
    template forEach(per: Amount, n: Nominal): Amount { body = countOf($n) }\n\
    primitive didDiscard(group: NounPhrase): PostModifier = DidDiscard\n\
    primitive max: AggregateOp = Max\n\
    primitive manaValue: ProjAxis = ManaValue\n\
    primitive aggregateOver(op: AggregateOp, domain: Nominal, body: Amount): Amount = AggregateOver\n\
    primitive superlative(op: AggregateOp, axis: ProjAxis, domain: Nominal): PostModifier = Superlative\n\
    primitive atLeast: Comparator = AtLeast\n\
    primitive compareOver(domain: Nominal, measure: Amount, comparator: Comparator, bound: Amount): Nominal = CompareOver\n\
    primitive headed(subject: NounPhrase, head: Nominal): NounPhrase = Headed\n\
    primitive always: Condition = Always\n\
    primitive nothing: VerbPhrase = Nothing\n";

#[test]
fn agreement_postmodifier_requires_a_plural_group_determiner() {
    let preamble = format!("{NOMINAL_PREAMBLE}{AGREEMENT_PREAMBLE}");

    // Every determiner, plurality, slice count and quantity form that admits
    // more than one member, and a determiner or quantity left to a Template's
    // caller.
    let valid = format!(
        "{preamble}\
         template okAny: NounPhrase {{ body = counted(range(low = 0), nominal(card, post = [withDifferentNames])) }}\n\
         template okTwo: NounPhrase {{ body = counted(exactlyOf(2), nominal(card, post = [withDifferentNames])) }}\n\
         template okChosen: NounPhrase {{ body = counted(exactlyOf(chosenNumber), nominal(card, post = [withDifferentNames])) }}\n\
         template okUpToSix: NounPhrase {{ body = targets(range(high = 6), nominal(creature, post = [withDifferentNames])) }}\n\
         template okUpToTwoOf: NounPhrase {{ body = targets(upToOf(2), nominal(creature, post = [withDifferentNames])) }}\n\
         template okMoreThanOne: NounPhrase {{ body = counted(moreThanOf(1), nominal(card, post = [withDifferentNames])) }}\n\
         template okMoreThanZero: NounPhrase {{ body = counted(moreThanOf(0), nominal(card, post = [withDifferentNames])) }}\n\
         template okAdditionalTwo: NounPhrase {{ body = counted(additional(exactlyOf(2)), nominal(card, post = [withDifferentNames])) }}\n\
         template okWorthThree: NounPhrase {{ body = counted(worth(3), nominal(card, post = [withDifferentNames])) }}\n\
         template okAll: NounPhrase {{ body = allOf(nominal(card, post = [withDifferentNames])) }}\n\
         template okPluralThe: NounPhrase {{ body = thePlural(nominal(card, post = [withDifferentNames])) }}\n\
         template okBare: NounPhrase {{ body = bare(nominal(card, post = [withDifferentNames])) }}\n\
         template okWholeSlice(g: NounPhrase): NounPhrase {{\n\
           body = someOf(whole, description = nominal(card, post = [withDifferentNames]), $g)\n\
         }}\n\
         template okTotaling(g: NounPhrase): NounPhrase {{\n\
           body = someOf(totaling(4), description = nominal(card, post = [withDifferentNames]), $g)\n\
         }}\n\
         template okCountedSlice(g: NounPhrase): NounPhrase {{\n\
           body = someOf(sliceCounted(exactlyOf(3)), description = nominal(card, post = [withDifferentNames]), $g)\n\
         }}\n\
         template okNestedInSingular: NounPhrase {{\n\
           body = a(nominal(creature, post = [didDiscard(counted(exactlyOf(2), nominal(card, post = [withDifferentNames])))]))\n\
         }}\n\
         template okDeterminerParameter(d: DetPhrase): NounPhrase {{\n\
           body = described($d, nominal(card, post = [withDifferentNames]))\n\
         }}\n\
         template okQuantityParameter(q: Quantity): NounPhrase {{\n\
           body = counted($q, nominal(card, post = [withDifferentNames]))\n\
         }}\n\
         template okPossessive(owner: NounPhrase): NounPhrase {{\n\
           body = described(possessive($owner), nominal(card, post = [withDifferentNames]))\n\
         }}\n\
         template okForEach: Amount {{ body = forEach(1, nominal(card, post = [withDifferentNames])) }}\n\
         template okAggregate: Amount {{ body = aggregateOver(max, nominal(card, post = [withDifferentNames]), 1) }}\n\
         template okSuperlative: Nominal {{\n\
           body = nominal(creature, post = [superlative(max, manaValue, nominal(card, post = [withDifferentNames]))])\n\
         }}\n\
         template okCompareOver: Nominal {{\n\
           body = compareOver(nominal(card, post = [withDifferentNames]), 1, atLeast, 2)\n\
         }}"
    );
    assert_eq!(kinds(&valid), []);

    // Every form that admits at most one member, a consumer that forms no
    // group, and a Template whose caller supplies a singular determiner.
    let invalid = format!(
        "{preamble}\
         template badA: NounPhrase {{ body = a(nominal(card, post = [withDifferentNames])) }}\n\
         template badEach: NounPhrase {{ body = each(nominal(card, post = [withDifferentNames])) }}\n\
         template badThe: NounPhrase {{ body = the(nominal(card, post = [withDifferentNames])) }}\n\
         template badTarget: NounPhrase {{ body = target(nominal(creature, post = [withDifferentNames])) }}\n\
         template badUpToOne: NounPhrase {{ body = targets(range(high = 1), nominal(creature, post = [withDifferentNames])) }}\n\
         template badUpToOneOf: NounPhrase {{ body = targets(upToOf(1), nominal(creature, post = [withDifferentNames])) }}\n\
         template badCountOne: NounPhrase {{ body = counted(exactlyOf(1), nominal(card, post = [withDifferentNames])) }}\n\
         template badAdditionalOne: NounPhrase {{ body = counted(additional(exactlyOf(1)), nominal(card, post = [withDifferentNames])) }}\n\
         template badTheLast: NounPhrase {{ body = counted(theLast, nominal(card, post = [withDifferentNames])) }}\n\
         template badWorthOne: NounPhrase {{ body = counted(worth(1), nominal(card, post = [withDifferentNames])) }}\n\
         template badSliceOne(g: NounPhrase): NounPhrase {{\n\
           body = someOf(sliceCounted(exactlyOf(1)), description = nominal(card, post = [withDifferentNames]), $g)\n\
         }}\n\
         template differentCards(d: DetPhrase): NounPhrase {{\n\
           body = described($d, nominal(card, post = [withDifferentNames]))\n\
         }}\n\
         template badThroughParameter: NounPhrase {{ body = differentCards(each) }}\n\
         template badHeaded(x: NounPhrase): NounPhrase {{ body = headed($x, nominal(card, post = [withDifferentNames])) }}\n\
         template differentUpTo(p: Nat): NounPhrase {{\n\
           body = counted(range(high = $p), nominal(card, post = [withDifferentNames]))\n\
         }}\n\
         template okUpToThree: NounPhrase {{ body = differentUpTo(3) }}\n\
         template badUpToOneThroughParameter: NounPhrase {{ body = differentUpTo(1) }}\n\
         template differentWorth(p: Nat): NounPhrase {{\n\
           body = counted(worth($p), nominal(card, post = [withDifferentNames]))\n\
         }}\n\
         template okWorthThree: NounPhrase {{ body = differentWorth(3) }}\n\
         template badWorthOneThroughParameter: NounPhrase {{ body = differentWorth(1) }}\n\
         sba badRule {{\n\
           body = {{ scope = nominal(creature, post = [withDifferentNames]), when = always, then = nothing }}\n\
         }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &invalid)]),
        [
            vec![(RefusalKind::AgreementWithoutGroup, s("withDifferentNames")); 11],
            // Inside a Template's body, at the call that supplies the determiner.
            vec![(
                RefusalKind::AgreementWithoutGroup,
                s("differentCards(each)")
            )],
            vec![(RefusalKind::AgreementWithoutGroup, s("withDifferentNames"))],
            vec![(RefusalKind::AgreementWithoutGroup, s("differentUpTo(1)"))],
            vec![(RefusalKind::AgreementWithoutGroup, s("differentWorth(1)"))],
            vec![(RefusalKind::AgreementWithoutGroup, s("withDifferentNames"))],
        ]
        .concat()
    );
}

#[test]
fn stacked_refuses_a_coordinator_it_does_not_classify() {
    let source = format!(
        "{NOMINAL_PREAMBLE}\
         primitive nor: Coordinator = Nor\n\
         template neither: Nominal {{ body = nominal(pre = [coord(nor, [white, black])], card) }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [(RefusalKind::UnclassifiedCoordinator(s("nor")), s("nor"))]
    );
}

#[test]
fn a_definition_body_is_checked_with_each_calls_arguments() {
    // An `action` whose body leaves the count open: only the call that
    // passes 1 puts Agreement under a singular count.
    let source = format!(
        "{NOMINAL_PREAMBLE}{AGREEMENT_PREAMBLE}\
         primitive discard(group: NounPhrase): VerbPhrase = Discard\n\
         action discardDifferent(n: Amount) {{\n\
           body = discard(counted(exactlyOf($n), nominal(card, post = [withDifferentNames])))\n\
         }}\n\
         template discardTwo: VerbPhrase {{ body = discardDifferent(2) }}\n\
         template discardOne: VerbPhrase {{ body = discardDifferent(1) }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [(RefusalKind::AgreementWithoutGroup, s("discardDifferent(1)"))]
    );
}

#[test]
fn agreement_refuses_a_number_form_it_does_not_classify() {
    let source = format!(
        "{NOMINAL_PREAMBLE}{AGREEMENT_PREAMBLE}\
         primitive fewest: DetPhrase = Fewest\n\
         template cards: NounPhrase {{ body = described(fewest, nominal(card, post = [withDifferentNames])) }}\n\
         primitive most: Quantity = Most\n\
         template most: NounPhrase {{ body = counted(most, nominal(card, post = [withDifferentNames])) }}\n\
         primitive half: SliceCount = Half\n\
         template halfOf(g: NounPhrase): NounPhrase {{\n\
           body = someOf(half, description = nominal(card, post = [withDifferentNames]), $g)\n\
         }}\n\
         primitive several: Plurality = Several\n\
         template theSeveral: NounPhrase {{ body = described(the(plur = several), nominal(card, post = [withDifferentNames])) }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [
            (
                RefusalKind::UnclassifiedNumber {
                    word: s("fewest"),
                    sort: s("DetPhrase"),
                },
                s("fewest")
            ),
            (
                RefusalKind::UnclassifiedNumber {
                    word: s("most"),
                    sort: s("Quantity"),
                },
                s("most")
            ),
            (
                RefusalKind::UnclassifiedNumber {
                    word: s("half"),
                    sort: s("SliceCount"),
                },
                s("half")
            ),
            (
                RefusalKind::UnclassifiedNumber {
                    word: s("several"),
                    sort: s("Plurality"),
                },
                s("several")
            ),
        ]
    );
}

#[test]
fn own_name_and_shortened_name_are_refused_in_builtin() {
    let source = "\
        primitive ownName: NounPhrase = OwnName { spelling = \"#\" }\n\
        template shortenedName: NounPhrase { spelling = \"##\", body = ownName }\n\
        primitive see(who: NounPhrase): VerbPhrase = See\n\
        template badOwn: VerbPhrase { body = see(ownName) }\n\
        template badShort: VerbPhrase { body = see(shortenedName) }\n\
        template badDefault(who: NounPhrase = ownName): VerbPhrase { body = see($who) }\n";
    assert_eq!(
        refusals(&[("a.syl", source)]),
        [
            (RefusalKind::OwnNameInBuiltin(s("ownName")), s("ownName")),
            (
                RefusalKind::OwnNameInBuiltin(s("shortenedName")),
                s("shortenedName")
            ),
            (RefusalKind::OwnNameInBuiltin(s("ownName")), s("ownName")),
        ]
    );
}

#[test]
fn shortened_name_requires_a_face_with_a_shortened_name() {
    let builtin_src = "\
        primitive true: Bool = True\n\
        primitive false: Bool = False\n\
        supertype legendary { name = \"Legendary\" }\n\
        card_type creature { name = \"Creature\", permanentType = true }\n\
        card_type land { name = \"Land\", permanentType = true }\n\
        card_type sorcery { name = \"Sorcery\", permanentType = false }\n\
        primitive ownName: NounPhrase = OwnName { spelling = \"#\" }\n\
        template shortenedName: NounPhrase { spelling = \"##\", body = ownName }\n\
        primitive spell(who: NounPhrase): Ability = Spell\n\
        primitive bandAt(n: Nat): LevelRange = BandAt\n\
        primitive singleFaced(face: Characteristics): Card = SingleFaced\n\
        primitive transforming(front: Characteristics, back: Characteristics): Card = Transforming\n\
        primitive leveler(inner: Characteristics, bands: List(LevelBand)): Card = Leveler\n\
        primitive sharedLineSplit(\n\
          shared: Characteristics,\n\
          left: SharedLineHalf,\n\
          right: SharedLineHalf): Card = SharedLineSplit\n";
    let builtin = bundle("builtin", FileId(0), &[("builtin.syl", builtin_src)]);

    let valid_canon = "\
        card singleFaced({\n\
          name = \"Grizzly Bears\",\n\
          types = [creature],\n\
          text = [spell(ownName)] })\n\
        card singleFaced({\n\
          name = \"Urza's Ruinous Blast\",\n\
          supertypes = [legendary],\n\
          types = [sorcery],\n\
          text = [spell(ownName)] })\n\
        card singleFaced({\n\
          name = \"Urza, Lord High Artificer\",\n\
          supertypes = [legendary],\n\
          types = [creature],\n\
          text = [spell(ownName), spell(shortenedName)] })\n\
        card singleFaced({\n\
          name = \"Tor Wauki the Younger\",\n\
          supertypes = [legendary],\n\
          types = [creature],\n\
          text = [spell(shortenedName)] })\n\
        card singleFaced({\n\
          name = \"Captain James T. Kirk\",\n\
          supertypes = [legendary],\n\
          types = [creature],\n\
          text = [spell(shortenedName)] })\n\
        card transforming(\n\
          { name = \"Civilized Scholar\", types = [creature], text = [spell(ownName)] },\n\
          { name = \"Homicidal Brute, Unbound\", supertypes = [legendary], types = [creature], text = [spell(shortenedName)] })\n\
        card leveler(\n\
          { name = \"Kargan, Dragonlord\", supertypes = [legendary], types = [creature] },\n\
          [{ range = bandAt(1), text = [spell(shortenedName)] }])\n\
        card sharedLineSplit(\n\
          { supertypes = [legendary], types = [creature] },\n\
          { name = \"Alpha, the First\", text = [spell(shortenedName)] },\n\
          { name = \"Omega\", text = [spell(ownName)] })\n";
    let ok_bundle = bundle("canon", builtin.next_file_id(), &[("ok.syl", valid_canon)]);
    check(&builtin, &[&ok_bundle]).unwrap_or_else(|e| panic!("expected ok, got: {e}"));

    let invalid_canon = "\
        card singleFaced({\n\
          name = \"Grizzly Bears\",\n\
          types = [creature],\n\
          text = [spell(shortenedName)] })\n\
        card singleFaced({\n\
          name = \"Urza's Ruinous Blast\",\n\
          supertypes = [legendary],\n\
          types = [sorcery],\n\
          text = [spell(shortenedName)] })\n\
        card singleFaced({\n\
          name = \"Progenitus\",\n\
          supertypes = [legendary],\n\
          types = [creature],\n\
          text = [spell(shortenedName)] })\n\
        card singleFaced({\n\
          name = \"The First Sliver\",\n\
          supertypes = [legendary],\n\
          types = [creature],\n\
          text = [spell(shortenedName)] })\n\
        card singleFaced({\n\
          name = \"The Grey Havens\",\n\
          supertypes = [legendary],\n\
          types = [land],\n\
          text = [spell(shortenedName)] })\n\
        card transforming(\n\
          { name = \"Urza, Lord High Artificer\", supertypes = [legendary], types = [creature], text = [spell(shortenedName)] },\n\
          { name = \"Construct\", types = [creature], text = [spell(shortenedName)] })\n\
        card leveler(\n\
          { name = \"Student of Warfare\", types = [creature] },\n\
          [{ range = bandAt(1), text = [spell(shortenedName)] }])\n\
        card sharedLineSplit(\n\
          { supertypes = [legendary], types = [creature] },\n\
          { name = \"Alpha, the First\", text = [spell(shortenedName)] },\n\
          { name = \"Omega\", text = [spell(shortenedName)] })\n";
    let bad_bundle = bundle(
        "canon",
        builtin.next_file_id(),
        &[("bad.syl", invalid_canon)],
    );
    let Err(CheckError::Resolution { refusals, .. }) = check(&builtin, &[&bad_bundle]) else {
        panic!("expected resolution refusals");
    };
    let got: Vec<RefusalKind> = refusals.into_iter().map(|r| r.kind).collect();
    assert_eq!(
        got,
        [
            RefusalKind::ShortenedNameWithoutShortening {
                word: s("shortenedName"),
                face: s("Grizzly Bears"),
            },
            RefusalKind::ShortenedNameWithoutShortening {
                word: s("shortenedName"),
                face: s("Urza's Ruinous Blast"),
            },
            RefusalKind::ShortenedNameWithoutShortening {
                word: s("shortenedName"),
                face: s("Progenitus"),
            },
            RefusalKind::ShortenedNameWithoutShortening {
                word: s("shortenedName"),
                face: s("The First Sliver"),
            },
            RefusalKind::ShortenedNameWithoutShortening {
                word: s("shortenedName"),
                face: s("The Grey Havens"),
            },
            RefusalKind::ShortenedNameWithoutShortening {
                word: s("shortenedName"),
                face: s("Construct"),
            },
            RefusalKind::ShortenedNameWithoutShortening {
                word: s("shortenedName"),
                face: s("Student of Warfare"),
            },
            RefusalKind::ShortenedNameWithoutShortening {
                word: s("shortenedName"),
                face: s("Omega"),
            },
        ]
    );
}
