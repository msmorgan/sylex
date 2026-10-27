keyword escape (cost : Cost) : Ability := {
  lexical := {
    lemma := "escape",
    parameters := [[DashNonManaCost]],
    verbs := [{ lemma := "escape", frames := [Intransitive, Prepositional (["with"])] }]
  },
  spelling := "escape—<cost>.",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.can (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.you)
          (Semantics.Definitions.cast Semantics.Templates.NounPhrase.this_ (Option.some Semantics.Templates.Location.yourGraveyard)
            (Semantics.Templates.PlayPayment.payingInstead cost)))]
}
