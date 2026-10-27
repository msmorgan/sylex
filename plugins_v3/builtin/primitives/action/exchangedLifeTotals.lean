primitive exchangedLifeTotals (parties : Semantics.NounPhrase) : Semantics.Exchanged := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Exchanged.lifeTotals parties
}
