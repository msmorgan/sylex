keyword cleave (cost : Cost) : Ability := {
  lexical := { lemma := "cleave", parameters := [[Cost]], bracketed_text_removable := true },
  spelling := "cleave <cost>",
  body :=
    [static (altCost this_ (some cost)),
      static
        (onlyWhile
          (textChange thisSpell removeBracketed)
          (keywordCostPaid cleave.label thisSpell))]
}
