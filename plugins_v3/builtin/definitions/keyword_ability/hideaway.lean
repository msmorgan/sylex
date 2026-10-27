/-- "Hideaway N" [CR#702.75a]. "The player who controls the permanent that exiled this card may
look at this card": whoever controls it now, since a resolved effect that changes no
characteristics or control modifies the rules, so it can reach a player it did not reach when it
began [CR#611.2c]. The player who
looked at the card and exiled it face down may keep looking at it too [CR#406.3]. -/
keyword hideaway (amount : Amount) : Ability := {
  lexical := { lemma := "hideaway", parameters := [[Amount]] },
  spelling := "hideaway <amount>",
  body :=
    let exiledCard : NounPhrase := the (and [was (exile which), isCard])
    [ when (does thisPermanent enter)
          (sequentially
            [ lookAt (topSlice amount),
              exileFaceDown (someOf (oneThrough 1) them),
              .move theRest .anyLocation onBottom .arrival
                [.inOrder .randomOrder],
              establishThroughout
                (.conjunction none
                  [ visibility .lookAt you (objects exiledCard),
                    visibility .lookAt (controllerOf thisPermanent) (objects exiledCard) ])
                (forAsLongAs (is exiledCard (in_ exileZone))) ]) ]
}
