/-- "put <subject> onto the battlefield tapped and attacking [<defender>]"; with no defender
named, its controller chooses [CR#508.4]. -/
-- [CR#508.4]
template putOntoBattlefieldTappedAttacking (subject : NounPhrase)
    (from_ : Location := wherever) (defender : Option NounPhrase := none) : Action := {
  -- TODO(spelling): Context-dependent wording: put <subject> onto the battlefield tapped and attacking [<defender>]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.Action.zoneChange subject Semantics.Templates.Location.battlefield from_
      [Semantics.Templates.MoveRider.entersAs Semantics.Status.tapped, Semantics.Templates.MoveRider.entersAttacking defender]
}
