/-- "the amount of mana spent to cast <spell>" -/
template manaValueSpentToCast (spell : NounPhrase) : Amount := {
  spelling := "the amount of mana spent to cast <spell>",
  body :=
    Semantics.Templates.Amount.paid Semantics.Templates.LinkedFacet.linkedFacetManaValueSpent spell
}
