/-- "<subject> is <quality>" as a Characteristic-Defining Ability: "this creature is all
colors", "this object is colorless", "this object is every creature type"
[CR#604.3a,702.73a,702.114a]. -/
-- [CR#604.3a,702.73a,702.114a]
template qualityDefinition (subject : NounPhrase) (op : QualityOp)
    (payload : QualityPayload) : StaticSpec := {
  -- TODO(spelling): The op and payload select the characteristic definition's quality clause,
  -- including type or color structure.
  body :=
    characteristicDefining subject (Semantics.QualityPayload.edits op payload)
}
