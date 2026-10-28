/-- "put <subject> onto the battlefield tapped and attacking [<defender>]"; with no defender
named, its controller chooses [CR#508.4]. -/
-- [CR#508.4]
template putOntoBattlefieldTappedAttacking (subject : NounPhrase)
    (from_ : Location := wherever) (defender : Option NounPhrase := none) : Action := {
  -- TODO(spelling): Context-dependent wording: put <subject> onto the battlefield tapped and attacking [<defender>]; string substitution cannot express its omitted or inflected parts.
  body :=
    zoneChange subject battlefield from_
      [entersAs .tapped, entersAttacking defender]
}
