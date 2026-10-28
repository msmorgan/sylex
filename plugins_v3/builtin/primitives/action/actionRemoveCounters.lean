/-- "remove <quantity> <kind> counters from <from>" [CR#122.1]. -/
primitive actionRemoveCounters (quantity : Option Quantity) (kind : Option CounterKindSource) (from_ : NounPhrase) : Action := {
  body := Semantics.Primitives.Action.removeCounters quantity kind from_
}
