/-- "to activate abilities [of <source>]" -/
primitive toActivate (source : Option Predicate) : SpendPurpose := {
  -- TODO(spelling): Context-dependent wording: to activate abilities [of <source>]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.SpendPurpose.toActivate source
}
