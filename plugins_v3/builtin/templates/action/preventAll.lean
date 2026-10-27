/-- "Prevent all <kind> damage that would be dealt [to <to>] [duration]": a prevention effect,
a replacement of each such damage event by its prevention [CR#615.1]. `to` left `any` is damage
dealt to anything. -/
-- [CR#615.1]
template preventAll (to : NounPhrase) (duration : Option Duration)
    (kind : Option DamageKind := none) : Action := {
  -- TODO(spelling): Context-dependent wording: Prevent all <kind> damage that would be dealt [to <to>] [duration]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.Action.establish
      (Semantics.Templates.StaticSpec.replace
        (Semantics.Templates.WouldClause.would Semantics.Templates.Subject.any (Semantics.Templates.Action.dealDamage Semantics.Templates.Amount.anyAmount to kind))
        (Semantics.Templates.Replacement.instead (Semantics.Templates.Action.preventDamage Semantics.Templates.PreventCut.allDamage Option.none Option.none))
        Option.none Semantics.ReplUse.repeatedly Option.none)
      duration
}
