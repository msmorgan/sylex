/-- "{1} for each …": generic mana cost scaled by an amount. -/
template scaledGenericMana (amount : Amount) : Cost := {
  -- TODO(spelling): Context-dependent wording: {1} for each …; string substitution cannot express its omitted or inflected parts.
  body :=
    scaled (mana [generic 1]) amount
}
