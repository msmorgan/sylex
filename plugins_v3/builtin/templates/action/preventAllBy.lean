/-- "Prevent all <kind> damage that would be dealt [to <to>] by <source> [duration]" [CR#615.1]. -/
-- [CR#615.1]
template preventAllBy (source : NounPhrase) (to : NounPhrase)
    (duration : Option Duration) (kind : Option DamageKind := none) : Action := {
  -- TODO(spelling): Context-dependent wording: Prevent all <kind> damage that would be dealt [to <to>] by <source> [duration]; string substitution cannot express its omitted or inflected parts.
  body :=
    establish
      (replace
        (would (by_ source)
          (dealDamage anyAmount to kind))
        (instead (preventDamage allDamage none none))
        none Semantics.ReplUse.repeatedly none)
      duration
}
