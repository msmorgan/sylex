keyword cleave (cost : Cost) : Ability := {
  lexical := { lemma := "cleave", parameters := [[Cost]], bracketed_text_removable := true },
  spelling := "cleave <cost>",
  body :=
    [Semantics.Templates.Ability.static (Semantics.Templates.StaticSpec.altCost Semantics.Templates.NounPhrase.this_ (Option.some cost)),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.onlyWhile
          (Semantics.Templates.StaticSpec.textChange Semantics.Templates.NounPhrase.thisSpell Semantics.Templates.TextChange.removeBracketed)
          (Semantics.Templates.Condition.keywordCostPaid Semantics.Definitions.cleave.label Semantics.Templates.NounPhrase.thisSpell))]
}
