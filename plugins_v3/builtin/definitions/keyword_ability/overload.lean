keyword overload (cost : Cost) : Ability := {
  lexical := { lemma := "overload", parameters := [[Cost]] },
  spelling := "overload <cost>",
  body :=
    [static (altCost this_ (some cost)),
      static
        (onlyWhile
          (textChange thisSpell targetToEach)
          (keywordCostPaid overload.label thisSpell))]
}
