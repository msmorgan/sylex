/-- "unspent [<type>] mana" -/
primitive unspent (type : Option ColorOrColorless) : ManaHeld := {
  -- TODO(spelling): Context-dependent wording: unspent [<type>] mana; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.ManaHeld.unspent type
}
