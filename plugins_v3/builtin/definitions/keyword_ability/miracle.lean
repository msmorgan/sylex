/-- "Miracle [cost]": "You may reveal this card from your hand as you draw it if it's the first
card you've drawn this turn. When you reveal this card this way, you may cast it by paying [cost]
rather than its mana cost." [CR#702.94a] A static ability linked to a triggered one [CR#603.11],
written as a reflexive trigger: as you draw your first card each turn, if it is this card. -/
keyword miracle (cost : Cost) : Ability := {
  lexical := { lemma := "miracle", parameters := [[Cost]] },
  spelling := "miracle <cost>",
  body :=
    [ static
        (asEvent (nthOccurrence (.nth 1) (does you (draw 1)) (some .turn))
          (doIf (is (that .card) (not (otherThan thisCard)))
            (triggerReflexively (may (reveal thisCard))
              (may (cast thisCard (payment := payingInstead cost))))
            none)) ]
}
