-- [CR#701.66a]
action earthbend (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "earthbend", past := "earthbent", frames := [Amount, Intransitive] }] },
  spelling := "earthbend <amount>",
  body :=
    sequentially
        [ .establish
            (.characteristicChange (target (and [isLand, actorControls]))
              [ .typeLine .adds { types := some [.creature] },
                .stat .sets .power 0,
                .stat .sets .toughness 0,
                .addedAbilities [haste] ])
            none,
          putCounters amount p1p1Counter (that land),
          delay (does (that land) die)
            (returnToBattlefield (that land) [.entersAs .tapped, .under .actor])
            [does .any (.move (that land) battlefield exileZone .departure [])] ]
}
