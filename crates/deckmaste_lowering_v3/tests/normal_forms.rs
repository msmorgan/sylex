//! The downstream readings of the authoring constructors that replaced computed Template bodies
//! (`plugins-v3-first-order-templates`): each reads to what the Template computed before.
use std::path::Path;

use deckmaste_lowering_v3::reads::CopyExceptExpansionExt;
use deckmaste_lowering_v3::reads::CostPaymentExt;
use deckmaste_lowering_v3::reads::expand_edits;
use deckmaste_semantics_v3::ast::Action;
use deckmaste_semantics_v3::ast::CharacteristicEdit;
use deckmaste_semantics_v3::ast::CopyExcept;
use deckmaste_semantics_v3::ast::Cost;
use deckmaste_semantics_v3::ast::StaticSpec;
use deckmaste_semantics_v3::bundle::Bundle;
use deckmaste_semantics_v3::lower::Elaborator;

fn builtin() -> Bundle {
    static BUNDLE: std::sync::OnceLock<Bundle> = std::sync::OnceLock::new();
    BUNDLE
        .get_or_init(|| {
            Bundle::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins_v3/builtin"))
                .unwrap()
        })
        .clone()
}

fn paid_cost(reader: &Elaborator<'_>, action: &str) -> Cost {
    let Action::Pay { cost, .. } = reader.read::<Action>(action).unwrap() else {
        panic!("{action} is not a payment")
    };
    assert!(matches!(cost.as_ref(), Cost::AsPaid { .. }), "{action}");
    *cost
}

#[test]
fn a_payment_of_a_plain_payment_reads_as_that_payment() {
    let bundle = builtin();
    let reader = Elaborator::new(&bundle);
    for (action, payment) in [
        ("pay (payLife 1) once", "perform (loseLife 1)"),
        ("pay (inCaller (payLife 5)) once", "inCaller (perform (loseLife 5))"),
        (
            "pay (scaled (inCaller (payLife 2)) 1) once",
            "scaled (inCaller (perform (loseLife 2))) 1",
        ),
        ("pay [[1], payLife 2] once", "[[1], perform (loseLife 2)]"),
        (
            "pay (perform (sacrifice (a isCreature))) once",
            "perform (sacrifice (a isCreature))",
        ),
    ] {
        assert_eq!(
            paid_cost(&reader, action).payment(),
            reader.read::<Cost>(payment).unwrap(),
            "{action}"
        );
    }
}

#[test]
fn a_quality_payload_reads_as_the_edits_it_writes() {
    let bundle = builtin();
    let reader = Elaborator::new(&bundle);
    for (spec, edits) in [
        (
            "qualityChange thisLand sets (bundle { characteristics := { types := [creature], \
             power := 2, toughness := 2 }, qualities := [withEveryType creature] } (some land))",
            "[typeLine adds { types := some [creature] }, stat power (set 2), \
             stat toughness (set 2), quality sets (withEveryType creature)]",
        ),
        (
            "qualityChange thisLand loses (bundle { name := \"Sample\", colors := [blue], \
             text := [flying], power := 1 })",
            "[typeLine loses {}, name loses \"Sample\", colors loses (.some [blue]), \
             stat power (down 1), removedAbilities (specified [written flying])]",
        ),
        (
            "qualityDefinition thisCreature sets (colored every)",
            "[colors sets every]",
        ),
        (
            "qualityDefinition thisCreature adds (everyTypeOf creature)",
            "[quality adds (everyTypeOf creature)]",
        ),
    ] {
        let (StaticSpec::CharacteristicChange { edits: written, .. }
        | StaticSpec::CharacteristicDefining { edits: written, .. }) =
            reader.read::<StaticSpec>(spec).unwrap()
        else {
            panic!("{spec} is no characteristic edit")
        };
        assert!(
            matches!(written.as_slice(), [CharacteristicEdit::Payload { .. }]),
            "{spec}"
        );
        assert_eq!(
            expand_edits(&written),
            reader.read::<Vec<CharacteristicEdit>>(edits).unwrap(),
            "{spec}"
        );
    }
}

#[test]
fn copied_characteristics_read_as_the_exceptions_they_write() {
    let bundle = builtin();
    let reader = Elaborator::new(&bundle);
    for (written, exceptions) in [
        (
            "copyCharacteristics { subtypes := [spirit], power := 1, toughness := 1 } true",
            "[edits [typeLine adds { subtypes := some [spirit] }, stat power (set 1), \
             stat toughness (set 1)]]",
        ),
        (
            "copyCharacteristics { power := 3, text := [flying] }",
            "[edits [stat power (set 3)], alsoHas flying]",
        ),
        (
            "copyCharacteristics { types := [artifact], cost := [2] }",
            "[edits [typeLine sets { types := some [artifact] }, manaCost (some [2])]]",
        ),
        ("copyCharacteristics {}", "[]"),
    ] {
        let read = reader.read::<Vec<CopyExcept>>(written).unwrap();
        assert!(
            matches!(read.as_slice(), [CopyExcept::Characteristics { .. }]),
            "{written}"
        );
        assert_eq!(
            read.iter()
                .flat_map(CopyExceptExpansionExt::expanded)
                .collect::<Vec<_>>(),
            reader.read::<Vec<CopyExcept>>(exceptions).unwrap(),
            "{written}"
        );
    }
}

fn changed_edits(reader: &Elaborator<'_>, spec: &str) -> Vec<CharacteristicEdit> {
    let StaticSpec::CharacteristicChange { edits, .. } = reader.read::<StaticSpec>(spec).unwrap()
    else {
        panic!("{spec} is no characteristic change")
    };
    edits
}

#[test]
fn a_granted_keyword_term_reads_as_the_keyword_or_its_family_selection() {
    let bundle = builtin();
    let reader = Elaborator::new(&bundle);
    let bare = changed_edits(&reader, "keywordGrant thisCreature flying [isCreature]");
    assert!(matches!(bare.as_slice(), [CharacteristicEdit::AddedKeywordTerm { .. }]));
    assert_eq!(
        expand_edits(&bare),
        [CharacteristicEdit::AddedAbilities {
            abilities: vec![deckmaste_semantics_v3::ast::Ability::Keyword {
                keyword: reader
                    .read::<deckmaste_semantics_v3::ast::KeywordLabel>("flying")
                    .unwrap(),
                params: Vec::new(),
                body: Vec::new(),
            }],
        }]
    );
    let family = changed_edits(
        &reader,
        "keywordGrant thisCreature (anyIn { word := landwalk }) [isCreature, in_ graveyard]",
    );
    assert_eq!(
        expand_edits(&family),
        changed_edits(
            &reader,
            "abilityGrantSelected thisCreature (allOf [isCreature, in_ graveyard]) \
             (specified [abilities (anyIn { word := landwalk })])",
        )
    );
}
