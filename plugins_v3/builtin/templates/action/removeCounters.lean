/-- "remove <q> <kind> counters from <from>" -/
template removeCounters (quantity : Quantity) (kind : Option CounterKindSource)
    (from_ : NounPhrase) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by kind.
  body := actionRemoveCounters (some quantity) kind from_
}
