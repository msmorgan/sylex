primitive actionRemoveCounters (quantity : Option Semantics.Quantity) (kind : Option Semantics.CounterKindSource) (from_ : Semantics.NounPhrase) : Semantics.Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.removeCounters quantity kind from_
}
