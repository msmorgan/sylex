/-- Green Sun's Twilight: "Reveal the top X plus one cards of your library. Choose a creature card
and/or a land card from among them. Put those cards into your hand and the rest on the bottom of
your library in a random order. If X is 5 or more, instead put the chosen cards onto the
battlefield or into your hand and the rest on the bottom of your library in a random order."
The cards are chosen as the effect applies [CR#608.2d]. A self-replacement [CR#614.15]: "the
chosen cards" reads back the cards the replaced action read, the ones chosen. -/
card := {
  name := "Green Sun's Twilight",
  cost := [«variable», green],
  types := [sorcery],
  text := [
    spell
      [
        reveal (topSlice (plus (letter x) 1)),
        choose
          (NounPhrase.andOr
             [
               fromAmong (exactly 1) [isCreature, isCard] them,
               fromAmong (exactly 1) [isLand, isCard] them
             ]),
        replace
          [putInto (those card) hand, putInto theRest onBottom (riders := [inOrder randomOrder])]
          (doIf (compareAmt (letter x) atLeast 5)
             (instead
                [
                  putInto (thePlural [isCard, isChosen]) (Templates.Location.or [battlefield, hand]),
                  putInto theRest onBottom (riders := [inOrder randomOrder])
                ]))
      ]
  ]
}
