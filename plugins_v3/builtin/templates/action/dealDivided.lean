/-- "<source> deals N damage divided as you choose among <among>" -/
template dealDivided (source : NounPhrase) (amount : Amount) (among : NounPhrase) :
    Action := {
  -- TODO(spelling): <source> deals N damage divided as you choose among <among>; wording varies with amount, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    distribute (dividedVerbDamage source) amount among
}
