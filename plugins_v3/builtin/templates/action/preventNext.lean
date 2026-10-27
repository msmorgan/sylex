/-- "Prevent the next N <kind> damage that would be dealt [to <to>] [duration]": a shield, used
up as it prevents [CR#615.7]. -/
-- [CR#615.7]
template preventNext (to : NounPhrase) (amount : Amount) (duration : Option Duration)
    (kind : Option DamageKind := none) : Action := {
  -- TODO(spelling): Context-dependent wording: Prevent the next N <kind> damage that would be dealt [to <to>] [duration]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.Action.establish
      (Semantics.Templates.StaticSpec.replace
        (Semantics.Templates.WouldClause.would Semantics.Templates.Subject.any (Semantics.Templates.Action.dealDamage Semantics.Templates.Amount.anyAmount to kind))
        (Semantics.Templates.Replacement.instead
          (Semantics.Templates.Action.preventDamage (Semantics.Templates.PreventCut.theNext amount) Option.none Option.none))
        Option.none Semantics.ReplUse.repeatedly Option.none)
      duration
}
