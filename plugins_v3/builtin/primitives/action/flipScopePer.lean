primitive flipScopePer (each : Semantics.NounPhrase) : Semantics.FlipScope := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.FlipScope.per each
}
