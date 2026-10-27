/-- "remove <q> <kind> counters from <from>" -/
primitive removeCounters (quantity : Quantity) (kind : Option CounterKindSource)
    (from_ : NounPhrase) : Action := {
  -- TODO(spelling): Context-dependent wording: remove <q> <kind> counters from <from>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Action.removeCounters (Option.some quantity) kind from_
}
