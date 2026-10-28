/-- "a <dom> with <stat> <r> <bound>": the stat of `member`, the member itself unless written. -/
template comparesOwnStat (stat : Stat) (domain : Predicate) (comparator : Comparator)
    (bound : Amount) (member : NounPhrase := ownMember) : Predicate := {
  -- TODO(spelling): Context-dependent wording: a <dom> with <stat> <r> <bound>; string substitution cannot express its omitted or inflected parts.
  body :=
    compareOver domain (statOf (.stat stat) member) comparator bound
}
