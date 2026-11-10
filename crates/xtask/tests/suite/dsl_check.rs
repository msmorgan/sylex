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

/// A trigger header names its event; alternative events are an `or` written
/// as that event, never a field beside it. `triggered` consumes the record
/// sort, which an unconsumed sort would otherwise make unknown.
#[test]
fn a_trigger_header_writes_its_event_and_no_alternatives() {
    assert_eq!(
        kinds(
            "primitive now: GameEvent = Now\n\
             primitive triggered(header: TriggerHeader): Ability = Triggered\n\
             template a: TriggerHeader { body = { event = now, while = none, timing = none } }\n\
             template b: TriggerHeader { body = { while = none, timing = none } }\n\
             template c: TriggerHeader {\n\
               body = { event = now, alternatives = [], while = none, timing = none }\n\
             }"
        ),
        [
            RefusalKind::MissingField {
                field: s("event"),
                at: s("TriggerHeader")
            },
            RefusalKind::UnknownField {
                field: s("alternatives"),
                at: s("TriggerHeader")
            },
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

#[test]
fn a_rules_row_writes_a_body_or_its_record_fields() {
    let preamble = "primitive yes: Predicate = Yes\n\
                    primitive always: Condition = Always\n\
                    primitive nothing: Action = Nothing\n";
    let source = format!(
        "{preamble}\
         sba a {{ scope = yes, when = always, then = nothing }}\n\
         sba b {{ body = {{ scope = yes, when = always, then = nothing }} }}\n\
         sba c {{ body = {{ scope = yes, when = always, then = nothing }}, scope = yes }}\n\
         sba d {{ scope = yes, when = always }}"
    );
    assert_eq!(
        kinds(&source),
        [
            RefusalKind::UnknownField {
                field: s("scope"),
                at: s("`sba` c")
            },
            RefusalKind::MissingField {
                field: s("then"),
                at: s("SbaRule")
            },
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

/// A subtype's `kind` is the list of card types it is correlated with: a
/// shared family names a Template at `List(CardType)` (Spirit and Arcane), a
/// single-type family a list literal (Siege). A lone card type is no list.
#[test]
fn a_subtype_kind_is_a_list_of_card_types() {
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
             template creature: List(CardType) { body = [creature, kindred] }\n\
             template spell: List(CardType) { body = [instant, sorcery] }\n\
             subtype spirit { kind = creature }\n\
             subtype arcane { kind = spell }\n\
             subtype siege { kind = [battle] }\n\
             subtype bad { kind = battle }"
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
