/-- Duel for Dominance: "Coven — Choose target creature you control and target creature you
don't control. If you control three or more creatures with different powers, put a +1/+1 counter
on the chosen creature you control. Then the chosen creatures fight each other." The targets are
chosen as the spell is cast [CR#601.2c]; "the chosen creature you control" reads back the first
of them, which its restriction picks out of the two, and "the chosen creatures" the pair, which
fight as in [CR#701.14a]. -/
card := {
  name := "Duel for Dominance",
  cost := [1, green],
  types := [instant],
  text := [
    coven
      (spell
         [
           (choose (both (target [isCreature, youControl]) (target [isCreature, youDontControl]))),
           doIf
             (compareAmt (distinctCount (value power) (allOf [isCreature, youControl])) atLeast 3)
             (putCounters 1 p1p1Counter (the [isCreature, isChosen, youControl])),
           fightEachOther (thePlural [isCreature, isChosen])
         ])
  ]
}
