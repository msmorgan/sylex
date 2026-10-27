/-- "{1} for each …": generic mana cost scaled by an amount. -/
template scaledGenericMana (amount : Amount) : Cost := {
  -- TODO(spelling): Context-dependent wording: {1} for each …; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.Cost.scaled (Semantics.Templates.Cost.mana [Semantics.Templates.ManaSymbol.generic 1]) amount
}
