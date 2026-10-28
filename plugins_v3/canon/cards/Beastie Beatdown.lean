/-- Beastie Beatdown: "Choose target creature you control and target creature an opponent
controls. / Delirium — If there are four or more card types among cards in your graveyard, put
two +1/+1 counters on the creature you control. / The creature you control deals damage equal to
its power to the creature an opponent controls." Three paragraphs, three spell abilities
[CR#113.2c], of one spell: its targets are announced for the spell as a whole [CR#601.2c] and its
instructions are followed in the order written [CR#608.2c], so "the creature you control" and
"the creature an opponent controls" in the later two read back the first one's targets. -/
card := {
  name := "Beastie Beatdown",
  cost := [red, green],
  types := [sorcery],
  text := [
    spell
      (choose (both (target [isCreature, youControl]) (target [isCreature, anOpponentControls]))),
    delirium
      (spell
         (doIf (compareAmt (distinctCount cardType (allOf (cardIn yourGraveyard))) atLeast 4)
            (putCounters 2 p1p1Counter (the [isCreature, youControl])))),
    spell (dealDamageOwnPower (the [isCreature, youControl]) (the [isCreature, anOpponentControls]))
  ]
}
