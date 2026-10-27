keyword overload (cost : Cost) : Ability := {
  lexical := { lemma := "overload", parameters := [[Cost]] },
  spelling := "overload <cost>",
  body :=
    [Semantics.Templates.Ability.static (Semantics.Templates.StaticSpec.altCost Semantics.Templates.NounPhrase.this_ (Option.some cost)),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.onlyWhile
          (Semantics.Templates.StaticSpec.textChange Semantics.Templates.NounPhrase.thisSpell Semantics.Templates.TextChange.targetToEach)
          (Semantics.Templates.Condition.keywordCostPaid Semantics.Definitions.overload.label Semantics.Templates.NounPhrase.thisSpell))]
}
