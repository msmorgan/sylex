//! The head-line child (`CompiledGrammar::head_line_child`) of real
//! Productions, one for each step of its rule. The fork tests read Readings
//! by the same rule, so they cannot catch a wrong one; these pin it.

use sylex_vocabulary::LexicalEnvironment;

/// (Production, the name of its head-line child), by step: a Linked
/// Conjunct's connective; a `head`; a coordination schema's coordinator, and
/// its serial tail; the only child; a `coordinator` the grammar writes; a
/// relative clause's `marker`; the first child bearing no dependent function
/// (`not`'s `filler`); the first child (a Complement Cluster's).
const HEAD_LINES: &[(&str, &str)] = &[
    ("LinkedClause", "adjunct"),
    ("Determined", "head"),
    ("Coordination:Clause#Coordinator/binary", "coordinator"),
    ("Coordination:Clause#Coordinator/serial", "rest"),
    ("BareRelative", "clause"),
    ("AddedElement", "coordinator"),
    ("SubjectRelative", "marker"),
    ("NegatedConjunct", "filler"),
    (
        "Valence:VerbPhrase/Ditransitive(NP,NP)/cluster",
        "indirect_object",
    ),
];

#[test]
fn head_line_children_of_real_productions() {
    let env = LexicalEnvironment::load().unwrap();
    let grammar = sylex_english::bind(&env.lexicon, &env.frames).unwrap();
    for &(id, child) in HEAD_LINES {
        let production = grammar
            .production_index(dpsi::Ident::new(id))
            .unwrap_or_else(|| panic!("no Production `{id}`"));
        let at = grammar
            .head_line_child(production)
            .unwrap_or_else(|| panic!("`{id}` has no head-line child"));
        assert_eq!(
            grammar.production_functions(production)[at],
            child,
            "`{id}`"
        );
    }
}
