/-- "remove all [<kind>] counters from <from>" -/
template removeAllCounters (kind : Option CounterKindSource) (from_ : NounPhrase) :
    Action := {
  -- TODO(spelling): Include or omit the clauses supplied by kind.
  body := actionRemoveCounters none kind from_
}
