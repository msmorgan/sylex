/-- "Prevent all <kind> damage that would be dealt [to <to>] [duration]": a prevention effect,
a replacement of each such damage event by its prevention [CR#615.1]. `to` left `any` is damage
dealt to anything. -/
-- [CR#615.1]
template preventAll (to : NounPhrase) (duration : Option Duration)
    (kind : Option DamageKind := none) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by duration, kind.
  body :=
    establish
      (replace
        (would any (dealDamage anyAmount to kind))
        (instead (preventDamage allDamage none none))
        none .repeatedly none)
      duration
}
