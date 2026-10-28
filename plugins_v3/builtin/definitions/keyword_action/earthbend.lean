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
            [typeLine .adds
                { types := some [Semantics.CardType.creature] },
              stat .sets .power 0,
              stat .sets .toughness 0,
              addedAbilities [haste]])
          none,
        putCounters amount (printedKind p1p1Counter)
          (that land),
        delay
          (does
            (actor (that land))
            die)
          (returnToBattlefield (that land)
            [entersAs .tapped, under actor])
          [does any
              (move (that land)
                battlefield (some exileZone)
                .departure [])]]
}
