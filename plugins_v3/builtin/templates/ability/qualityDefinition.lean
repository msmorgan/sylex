/-- "<subject> is <quality>" as a Characteristic-Defining Ability: "this creature is all
colors", "this object is colorless", "this object is every creature type"
[CR#604.3a,702.73a,702.114a]. -/
-- [CR#604.3a,702.73a,702.114a]
template qualityDefinition (subject : NounPhrase) (op : QualityOp)
    (payload : QualityPayload) : StaticSpec := {
  -- TODO(spelling): Context-dependent wording: <subject> is <quality>; string substitution cannot express its omitted or inflected parts.
  body :=
    staticSpecCharacteristicDefining subject (Semantics.QualityPayload.edits op payload)
}
