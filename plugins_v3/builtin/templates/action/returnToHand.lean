/-- "return <subject> [from <from_>] to its owner's hand" -/
template returnToHand (subject : NounPhrase) (from_ : Location := wherever) :
    Action := {
  -- TODO(spelling): Context-dependent wording: return <subject> [from <from_>] to its owner's hand; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.Action.zoneChange subject Semantics.Templates.Location.hand from_ []
}
