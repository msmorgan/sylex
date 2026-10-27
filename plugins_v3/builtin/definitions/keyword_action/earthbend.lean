-- [CR#701.66a]
action earthbend (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "earthbend", past := "earthbent", frames := [Amount, Intransitive] }] },
  spelling := "earthbend <amount>",
  body :=
    Semantics.Templates.Action.sequentially
      [Semantics.Templates.Action.establish
          (Semantics.Templates.StaticSpec.characteristicChange
            (Semantics.Templates.NounPhrase.target
              (Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isLand, Semantics.Templates.Predicate.actorControls]))
            [Semantics.Templates.CharacteristicEdit.characteristicEditTypeLine Semantics.QualityOp.adds
                { types := Option.some [Semantics.CardType.creature] },
              Semantics.Templates.CharacteristicEdit.characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.power 0,
              Semantics.Templates.CharacteristicEdit.characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.toughness 0,
              Semantics.Templates.CharacteristicEdit.characteristicEditAddedAbilities [Semantics.Definitions.haste]])
          Option.none,
        Semantics.Templates.Action.putCounters amount (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
          (Semantics.Templates.NounPhrase.that Semantics.Templates.NounWord.land),
        Semantics.Templates.Action.delay
          (Semantics.Templates.GameEvent.does
            (Semantics.Templates.Subject.actor (Semantics.Templates.NounPhrase.that Semantics.Templates.NounWord.land))
            Semantics.Templates.Action.die)
          (Semantics.Templates.Action.returnToBattlefield (Semantics.Templates.NounPhrase.that Semantics.Templates.NounWord.land)
            [Semantics.Templates.MoveRider.entersAs Semantics.Status.tapped, Semantics.Templates.MoveRider.under Semantics.Templates.NounPhrase.actor])
          [Semantics.Templates.GameEvent.does Semantics.Templates.Subject.any
              (Semantics.Templates.Action.actionMove (Semantics.Templates.NounPhrase.that Semantics.Templates.NounWord.land)
                Semantics.Templates.Location.battlefield (Option.some Semantics.Templates.Location.exileZone)
                Semantics.ObservationPoint.departure [])]]
}
