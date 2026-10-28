/-- "put <subject> onto the battlefield tapped and attacking [<defender>]"; with no defender
named, its controller chooses [CR#508.4]. -/
-- [CR#508.4]
template putOntoBattlefieldTappedAttacking (subject : NounPhrase)
    (from_ : Location := wherever) (defender : Option NounPhrase := none) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by defender.
  body :=
    zoneChange subject battlefield from_
      [entersAs tapped, entersAttacking defender]
}
