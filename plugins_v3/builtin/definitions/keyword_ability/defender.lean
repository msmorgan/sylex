keyword defender : Ability := {
  lexical := { lemma := "defender" },
  spelling := "defender",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.cant (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisCreature)
          (Semantics.Templates.Action.attack Option.none))]
}
