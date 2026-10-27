keyword recover (cost : Cost) : Ability := {
  lexical := { lemma := "recover", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): recover [mana cost] / recover—[verbal cost]; separator and terminal period
  -- depend on the Cost structure.
  body :=
    [Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does Semantics.Templates.Subject.any
          (Semantics.Templates.Action.actionMove (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isCreature) Semantics.Templates.Location.battlefield
            (Option.some Semantics.Templates.Location.yourGraveyard) Semantics.ObservationPoint.departure []))
        (Semantics.Templates.Action.may (Semantics.Templates.Action.pay cost Semantics.PayTimes.once)
          (Option.some (Semantics.Templates.Action.returnToHand Semantics.Templates.NounPhrase.this_ Semantics.Templates.Location.yourGraveyard))
          (Option.some (Semantics.Definitions.exile Semantics.Templates.NounPhrase.this_)))]
}
