/-- "remove all [<kind>] counters from <from>" -/
template removeAllCounters (kind : Option CounterKindSource) (from_ : NounPhrase) :
    Action := {
  -- TODO(spelling): Context-dependent wording: remove all [<kind>] counters from <from>; string substitution cannot express its omitted or inflected parts.
  body := actionRemoveCounters none kind from_
}
