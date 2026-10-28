/-- "Paradigm" represents two spell abilities [CR#702.192a]: "If this is the first time a spell
you control with this spell's name has resolved this game, at the beginning of each of your
precombat main phases for the rest of the game, create a copy of this object in exile. You may
cast the copy without paying its mana cost" and "Exile this spell." It is the first time when no
other spell you controlled with this spell's name resolved earlier this game. -/
keyword paradigm : Ability := {
  lexical := { lemma := "paradigm" },
  spelling := "paradigm",
  body :=
    [spell
        (doIf
          (not
            (did Subject.any
              (resolve
                (a
                  (and
                    [isSpell, isOther, youControl,
                      shares Semantics.QualitySort.cardName Semantics.Agreement.share
                        thisSpell])))
              (within thisGame)))
          (delayWithin (beginningOfYour Semantics.TurnPart.firstMain)
            restOfGame
            (sequentially
              [copy (Semantics.CopySort.inZone Semantics.Zone.exile) this_ 1 [],
                may
                  (cast (that Semantics.NounWord.copy) none
                    withoutPaying)]))),
      spell (exile thisSpell)]
}
