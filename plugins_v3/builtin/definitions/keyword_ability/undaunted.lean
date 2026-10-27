keyword undaunted : Ability := {
  lexical := { lemma := "undaunted" },
  spelling := "undaunted",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.costShift Semantics.Templates.NounPhrase.thisSpell
          (Semantics.Templates.CostShift.costShiftGeneric (Semantics.Templates.down (Semantics.Templates.Amount.forEach 1 Semantics.Templates.Predicate.isOpponent))
            Option.none))]
}
