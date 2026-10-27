/-- "Storm": "When you cast this spell, copy it for each other spell that was cast before it this
turn. If the spell has any targets, you may choose new targets for any of the copies."
[CR#702.40a] -/
keyword storm : Ability := {
  lexical := { lemma := "storm" },
  spelling := "storm",
  body :=
    [ when (does you (cast thisSpell))
        (sequentially
          [ .copy .fromStack thisSpell
              (eventCount
                (cast (a (and [isSpell, isOther])))
                (a .isPlayer) earlierThisTurn)
              [],
            may (.chooseNewTargets (.pro (.word .copy) .many)) ]) ]
}
