primitive dividedVerbCounters (kind : Semantics.CounterKind) : Semantics.DividedVerb := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.DividedVerb.counters kind
}
