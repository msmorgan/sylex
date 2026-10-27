primitive flipScopeCount (amount : Semantics.Amount) : Semantics.FlipScope := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.FlipScope.count amount
}
