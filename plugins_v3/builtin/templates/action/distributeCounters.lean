/-- "distribute N <kind> counters among <among>" -/
template distributeCounters (amount : Amount) (kind : CounterKind) (among : NounPhrase) :
    Action := {
  -- TODO(spelling): distribute N <kind> counters among <among>; wording varies with amount, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Templates.Action.distribute (Semantics.Templates.DividedVerb.dividedVerbCounters kind) amount among
}
