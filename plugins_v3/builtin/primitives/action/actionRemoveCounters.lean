primitive actionRemoveCounters (quantity : Option Quantity) (kind : Option CounterKindSource) (from_ : NounPhrase) : Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.removeCounters quantity kind from_
}
