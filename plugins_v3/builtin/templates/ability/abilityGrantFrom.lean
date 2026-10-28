/-- "<subject> has all <classes> abilities of <source> [except <except>]" -/
template abilityGrantFrom (subject source : NounPhrase)
    (classes : List AbilityClass := []) (except : Option Predicate := none) : StaticSpec := {
  -- TODO(spelling): Include or omit the clauses supplied by except. Format and coordinate the
  -- classes list entries, including the empty case.
  body :=
    characteristicChange subject [addedAbilitiesOf source classes except]
}
