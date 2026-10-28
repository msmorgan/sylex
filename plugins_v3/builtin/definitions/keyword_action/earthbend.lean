-- [CR#701.66a]
action earthbend (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "earthbend", past := "earthbent", frames := [Amount, Intransitive] }] },
  spelling := "earthbend <amount>",
  body :=
    sequentially
      [establish
          (characteristicChange
            (target
              (and [isLand, actorControls]))
            [characteristicEditTypeLine Semantics.QualityOp.adds
                { types := some [Semantics.CardType.creature] },
              characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.power 0,
              characteristicEditStat Semantics.QualityOp.sets Semantics.CharacteristicStat.toughness 0,
              characteristicEditAddedAbilities [haste]])
          none,
        putCounters amount (printedKind p1p1Counter)
          (that land),
        delay
          (does
            (actor (that land))
            die)
          (returnToBattlefield (that land)
            [entersAs Semantics.Status.tapped, under actor])
          [does Subject.any
              (actionMove (that land)
                battlefield (some exileZone)
                Semantics.ObservationPoint.departure [])]]
}
