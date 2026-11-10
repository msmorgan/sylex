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
         template m: Nat {{ body = pair(b = 2, 3) }}\n\
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
         template over: Nat {{ body = pair(1, 2, 3) }}\n\
         template unknown: Nat {{ body = pair(1, c = 2) }}\n\
         template twice: Nat {{ body = pair(1, a = 2) }}"
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
                    argument: s("a")
                },
                s("a = 2")
            ),
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
            "primitive yes: Predicate = Yes\n\
             primitive has(supertype: Supertype): Predicate = HasSupertype\n\
             supertype legendary { name = \"Legendary\", confers = [] }\n\
             supertype basic\n\
             supertype world { permanentType = yes }\n\
             primitive snow: Supertype = Snow\n\
             template t: Predicate { body = has(legendary) }"
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
    let preamble = "primitive yes: Predicate = Yes\n\
                    primitive always: Condition = Always\n\
                    primitive nothing: Action = Nothing\n";
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
    let preamble = "primitive yes: Predicate = Yes\n\
                    primitive always: Condition = Always\n\
                    primitive nothing: Action = Nothing\n\
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
            "primitive hasSubtypesOf(token: DefinedToken): Predicate = HasSubtypesOf\n\
             primitive yes: Predicate = Yes\n\
             token jaceToken { body = { characteristics = {} } }\n\
             template a: Predicate { body = hasSubtypesOf(jaceToken) }\n\
             template b: Predicate { body = hasSubtypesOf(yes) }\n\
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
             primitive nothing: Action = Nothing\n\
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
    let preamble = "primitive nothing: Action = Nothing\nability crew { }\n";
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

const RING: &str = "primitive ownScope(deed: Option(Deed), body: Action): Action = OwnScope\n\
                    primitive nothing: Action = Nothing\n\
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
                "template t: Action { body = ownScope(none, nothing) }"
            )
        ]),
        [(
            RefusalKind::ReservedConstructor(s("Action.OwnScope")),
            s("ownScope")
        )]
    );
    // Elsewhere the declaration is refused; its calls are not refused again.
    assert_eq!(
        kinds(RING),
        [RefusalKind::ReservedConstructor(s("Action.OwnScope"))]
    );
    // Only the builtin bundle's file is the Ring's.
    let builtin = bundle(
        "builtin",
        FileId(0),
        &[("a.syl", "primitive nothing: Action = Nothing")],
    );
    let other = bundle(
        "builtin",
        builtin.next_file_id(),
        &[(
            ring,
            "primitive ownScope(deed: Option(Deed), body: Action): Action = OwnScope",
        )],
    );
    let Err(CheckError::Resolution { refusals, .. }) = check(&builtin, &[&other]) else {
        panic!("the other bundle's Ring file is refused");
    };
    assert_eq!(
        refusals[0].kind,
        RefusalKind::ReservedConstructor(s("Action.OwnScope"))
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
         card transforming(front = {{ name = \"F\", colors = [white] }}, back = {{ name = \"B\", colors = [1] }})"
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
             card meld(front = { name = \"Bruna\" }, back = { name = \"Brisela\" })"
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
/// any of the three words. Most cards print "and" between them (the primitive
/// `and`): *Shrine of Limitless Power*'s "At the beginning of your upkeep and
/// whenever you cast a black spell", *Minsc & Boo, Timeless Heroes*'s "When
/// Minsc & Boo enters and at the beginning of your upkeep" and *Bartered Cow*'s
/// "When this creature dies and when you discard this card" resolve. *Tenuous
/// Truce* prints "or" (the Template `or` over `and`): "When you attack enchanted
/// opponent or a planeswalker they control or when they attack you or a
/// planeswalker you control", here with its first arm shortened to "when you
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
                     whenever(does(actor(you), cast(a(and([isSpell, hasColor(black)])))))]),\n\
                   action = putCounters(lit(1), printedKind(chargeCounter), thisArtifact))]})\n\
             card singleFaced({\n\
               name = \"Minsc & Boo, Timeless Heroes\",\n\
               types = [planeswalker],\n\
               text = [\n\
                 triggered(\n\
                   and([when(does(actor(thisPlaneswalker), enter)), at(beginningOfYour(upkeep))]),\n\
                   action = may(\n\
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
                   action = create(lit(1), defined(foodToken)))]})\n\
             card singleFaced({\n\
               name = \"Tenuous Truce\",\n\
               types = [enchantment],\n\
               subtypes = [aura],\n\
               text = [\n\
                 triggered(\n\
                   or([\n\
                     when(does(actor(you), attack(enchanted(player)))),\n\
                     when(\n\
                       does(\n\
                         actor(enchanted(player)),\n\
                         attack(youOr(a(and([isPlaneswalker, youControl]))))))]),\n\
                   action = sacrifice(thisAura))]})\n\
             card singleFaced({\n\
               name = \"Bartered Cow\",\n\
               types = [creature],\n\
               text = [\n\
                 triggered(\n\
                   and([when(does(actor(thisCreature), die)), does(actor(you), discard(thisCard))]),\n\
                   action = create(lit(1), defined(foodToken)))]})"
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

/// The three builtin coercions over their primitives, with a consumer of
/// `NounWord` so the sort is known.
const COERCIONS: &str = "primitive type(type: CardType): NounWord = Type\n\
     primitive subtype(subtype: Subtype): NounWord = Subtype\n\
     primitive lit(value: Int): Amount = Literal\n\
     primitive noun(word: NounWord): Predicate = Noun\n\
     coercion(type: CardType): NounWord { body = type($type) }\n\
     coercion(subtype: Subtype): NounWord { body = subtype($subtype) }\n\
     coercion(value: Int): Amount { body = lit($value) }\n\
     primitive true: Bool = True\n\
     card_type creature { permanentType = true }\n\
     subtype goblin { types = [creature] }\n";

#[test]
fn a_coercion_applies_at_a_sort_mismatch() {
    let source = format!(
        "{COERCIONS}\
         template word: NounWord {{ body = creature }}\n\
         template sub: NounWord {{ body = goblin }}\n\
         template int: Amount {{ body = -2 }}\n\
         template param(t: CardType): NounWord {{ body = $t }}\n\
         template items: List(NounWord) {{ body = [creature, goblin] }}\n\
         template wrapped: NounWord {{ body = type(creature) }}\n\
         template literal: Amount {{ body = lit(3) }}\n\
         template own: Int {{ body = 3 }}\n\
         template arg: Predicate {{ body = noun(goblin) }}"
    );
    assert_eq!(kinds(&source), []);
}

#[test]
fn a_coercion_composes_with_option() {
    let source = format!(
        "{COERCIONS}\
         template word: Option(NounWord) {{ body = creature }}\n\
         template int: Option(Amount) {{ body = 2 }}\n\
         template param(t: Subtype): Option(NounWord) {{ body = $t }}\n\
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
    // `CardType` to `NounWord` to `Predicate`: refused at load, and a card
    // type at `Predicate` is not coerced twice.
    let source = format!(
        "{COERCIONS}\
         coercion(word: NounWord): Predicate {{ body = noun($word) }}\n\
         template t: Predicate {{ body = creature }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [
            (
                RefusalKind::ChainedCoercion {
                    from: s("CardType"),
                    via: s("NounWord"),
                    to: s("Predicate")
                },
                s("coercion(word: NounWord): Predicate { body = noun($word) }")
            ),
            (
                RefusalKind::ChainedCoercion {
                    from: s("Subtype"),
                    via: s("NounWord"),
                    to: s("Predicate")
                },
                s("coercion(word: NounWord): Predicate { body = noun($word) }")
            ),
            (
                RefusalKind::UnknownWord {
                    word: s("creature"),
                    sort: s("Predicate")
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
                target: s("NounWord")
            },
            s("subtype creature { types = [creature] }")
        )]
    );
}

#[test]
fn a_word_at_a_target_and_at_a_sort_coerced_into_it_is_refused_with_no_call_site() {
    // A leftover head-noun Template beside its card type.
    let source = format!("{COERCIONS}template creature: NounWord {{ body = type(creature) }}");
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [(
            RefusalKind::AmbiguousWord {
                word: s("creature"),
                sorts: vec![s("CardType"), s("NounWord")],
                target: s("NounWord")
            },
            s("template creature: NounWord { body = type(creature) }")
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
    let source = "primitive ordinalOf(o: EventOrdinal): Predicate = OrdinalOf\n\
         primitive faceDownOf(f: FaceDown): Predicate = FaceDownOf\n\
         coercion(o: EventOrdinal): Predicate { body = ordinalOf($o) }\n\
         coercion(f: FaceDown): Predicate { body = faceDownOf($f) }";
    assert_eq!(
        refusals(&[("a.syl", source)]),
        [(
            RefusalKind::AmbiguousLiteral {
                literal: s("a record"),
                sorts: vec![s("EventOrdinal"), s("FaceDown")],
                target: s("Predicate")
            },
            s("coercion(f: FaceDown): Predicate { body = faceDownOf($f) }")
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
         template article: NounWord {{ body = type(creature) }}\n\
         coercion(a: Int, b: Int): Amount {{ body = lit($a) }}\n\
         coercion(a: Int = 1): Amount {{ body = lit($a) }}\n\
         coercion(a: CardType): NounWord {{ body = article }}\n\
         template articleOf(t: CardType): NounWord {{ body = type($t) }}\n\
         coercion(a: CardType): NounWord {{ body = articleOf($a) }}\n\
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
            RefusalKind::CoercionBody(s("NounWord")),
            RefusalKind::CoercionBody(s("NounWord")),
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
    // At an `Option(NounWord)` position the `Option` declaration would be
    // found before the coercion from `CardType` is tried.
    let source =
        format!("{COERCIONS}template creature: Option(NounWord) {{ body = type(creature) }}");
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [(
            RefusalKind::AmbiguousWord {
                word: s("creature"),
                sorts: vec![s("CardType"), s("Option(NounWord)")],
                target: s("Option(NounWord)")
            },
            s("template creature: Option(NounWord) { body = type(creature) }")
        )]
    );
}

#[test]
fn a_list_literal_reaching_a_target_from_two_list_sorts_is_refused_with_no_call_site() {
    let source = format!(
        "{COERCIONS}\
         primitive ofTypes(types: List(CardType)): Predicate = OfTypes\n\
         primitive ofSubtypes(subtypes: List(Subtype)): Predicate = OfSubtypes\n\
         coercion(t: List(CardType)): Predicate {{ body = ofTypes($t) }}\n\
         coercion(t: List(Subtype)): Predicate {{ body = ofSubtypes($t) }}"
    );
    assert_eq!(
        refusals(&[("a.syl", &source)]),
        [(
            RefusalKind::AmbiguousLiteral {
                literal: s("a list"),
                sorts: vec![s("List(CardType)"), s("List(Subtype)")],
                target: s("Predicate")
            },
            s("coercion(t: List(Subtype)): Predicate { body = ofSubtypes($t) }")
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
    let written = |action: &str| format!("template t: Action {{ body = {action} }}");
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
