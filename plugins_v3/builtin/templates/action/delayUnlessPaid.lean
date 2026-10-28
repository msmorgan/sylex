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
    sequentially
      [delay event action [] none,
        establish
          (specialAction none cost (establish
              (cant (actor
                  (.laterMention "delay.trigger" thisAbility)) trigger)
              none) (some player))
          (some (durationBetween none (some event)))]
}
