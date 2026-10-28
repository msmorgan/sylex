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
      the
        (and
          [was (exile which),
            isCard]);
    [when
        (does (actor thisPermanent) enter)
        (sequentially
          [lookAt (topSlice amount),
            exileFaceDown
              (someOf (oneThrough 1) them),
            actionMove theRest wherever
              (some onBottom) Semantics.ObservationPoint.arrival
              [inOrder Semantics.Arrangement.randomOrder],
            establishThroughout
              (staticSpecConjunction none
                [visibility Semantics.ExposeVerb.lookAt you
                    (objects exiledCard),
                  visibility Semantics.ExposeVerb.lookAt
                    (controllerOf thisPermanent)
                    (objects exiledCard)])
              (forAsLongAs
                (is exiledCard (in_ exileZone)))])]
}
