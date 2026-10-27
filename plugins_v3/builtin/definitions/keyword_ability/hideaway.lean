/-- "Hideaway N" [CR#702.75a]. "The player who controls the permanent that exiled this card may
look at this card": whoever controls it now, since a resolved effect that changes no
characteristics or control modifies the rules, so it can reach a player it did not reach when it
began [CR#611.2c]. The player who
looked at the card and exiled it face down may keep looking at it too [CR#406.3]. -/
keyword hideaway (amount : Amount) : Ability := {
  lexical := { lemma := "hideaway", parameters := [[Amount]] },
  spelling := "hideaway <amount>",
  body :=
    have exiledCard :=
      Semantics.Templates.NounPhrase.the
        (Semantics.Templates.Predicate.and
          [Semantics.Templates.Predicate.was (Semantics.Definitions.exile Semantics.Templates.NounPhrase.which),
            Semantics.Templates.Predicate.isCard]);
    [Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisPermanent) Semantics.Templates.Action.enter)
        (Semantics.Templates.Action.sequentially
          [Semantics.Templates.Action.lookAt (Semantics.Templates.NounPhrase.topSlice amount),
            Semantics.Definitions.exileFaceDown
              (Semantics.Templates.NounPhrase.someOf (Semantics.Templates.Quantity.oneThrough 1) Semantics.Templates.NounPhrase.them),
            Semantics.Templates.Action.actionMove Semantics.Templates.NounPhrase.theRest Semantics.Templates.Location.wherever
              (Option.some Semantics.Templates.Location.onBottom) Semantics.ObservationPoint.arrival
              [Semantics.Templates.MoveRider.inOrder Semantics.Arrangement.randomOrder],
            Semantics.Templates.Action.establishThroughout
              (Semantics.Templates.StaticSpec.staticSpecConjunction Option.none
                [Semantics.Templates.StaticSpec.visibility Semantics.ExposeVerb.lookAt Semantics.Templates.NounPhrase.you
                    (Semantics.Templates.VisibleThing.objects exiledCard),
                  Semantics.Templates.StaticSpec.visibility Semantics.ExposeVerb.lookAt
                    (Semantics.Templates.NounPhrase.controllerOf Semantics.Templates.NounPhrase.thisPermanent)
                    (Semantics.Templates.VisibleThing.objects exiledCard)])
              (Semantics.Templates.Duration.forAsLongAs
                (Semantics.Templates.Condition.is exiledCard (Semantics.Templates.Predicate.in_ Semantics.Templates.Location.exileZone)))])]
}
