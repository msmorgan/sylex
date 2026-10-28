/-- "<subject> gets <delta> to one stat" -/
template modification (subject : NounPhrase) (stat : CharacteristicStat)
    (delta : Delta Amount) : StaticSpec := {
  -- TODO(spelling): <subject> gets <delta> to one stat; wording varies with stat, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    characteristicChange subject [statEdit stat delta]
}
