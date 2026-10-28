/-- "return <subject> [from <from_>] to the battlefield [riders]" -/
template returnToBattlefield (subject : NounPhrase) (riders : List MoveRider := [])
    (from_ : Option Location := none) : Action := {
  -- TODO(spelling): Context-dependent wording: return <subject> [from <from_>] to the battlefield [riders]; string substitution cannot express its omitted or inflected parts.
  body :=
    zoneChange subject battlefield
      (Option.getD from_ wherever) riders
}
