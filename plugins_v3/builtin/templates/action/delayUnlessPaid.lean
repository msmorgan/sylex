/-- "<action> at <event> unless <player> pays <cost> before that step": the delayed trigger, and a
special action `player` may take until it would trigger, paying `cost` to stop it from triggering
[CR#116.2c]: the special action establishes that the delayed trigger can't trigger, with no
Duration, since the text states none. Once it has triggered, paying no longer stops it. The text names the delayed trigger nowhere, so the special action reads
it by its Binding Identity (`delay.trigger`). -/
-- [CR#116.2c]
template delayUnlessPaid (event : GameEvent) (action : Action) (player : NounPhrase)
    (cost : Cost) : Action := {
  spelling := "<action> at <event> unless <player> pays <cost> before that step",
  body :=
    Semantics.Templates.Action.sequentially
      [Semantics.Templates.Action.actionDelay event [] Option.none action,
        Semantics.Templates.Action.establish
          (Semantics.Templates.StaticSpec.staticSpecSpecialAction (Option.some player) Option.none cost
            (Semantics.Templates.Action.establish
              (Semantics.Templates.StaticSpec.staticSpecCant
                (Semantics.Templates.Subject.actor
                  (Semantics.NounPhrase.laterMention "delay.trigger" Semantics.Templates.NounPhrase.thisAbility))
                Semantics.Templates.Action.trigger { })
              Option.none))
          (Option.some (Semantics.Templates.Duration.durationBetween Option.none (Option.some event)))]
}
