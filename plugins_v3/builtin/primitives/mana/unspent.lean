/-- "unspent [<type>] mana" -/
primitive unspent (type : Option ColorOrColorless) : ManaHeld := {
  -- TODO(spelling): Include or omit the clauses supplied by type.
  body :=
    Semantics.Primitives.ManaHeld.unspent type
}
