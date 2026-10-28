/-- "<subject> gets <delta> to one stat" -/
template modification (subject : NounPhrase) (stat : CharacteristicStat)
    (delta : Delta Amount) : StaticSpec := {
  -- TODO(spelling): Select the clause for raising, lowering, or setting delta on stat; setting is
  -- not a 'gets' modifier.
  body :=
    characteristicChange subject [statEdit stat delta]
}
