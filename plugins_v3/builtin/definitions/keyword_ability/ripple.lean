/-- "Ripple N" with its definition [CR#702.60a]: "When you cast this spell, you may reveal the top
N cards of your library. If you do, you may cast any of those cards with the same name as this
spell without paying their mana costs, then put all revealed cards not cast this way on the bottom
of your library in any order." -/
keyword ripple (amount : Amount) : Ability := {
  lexical := { lemma := "ripple", parameters := [[Amount]] },
  spelling := "ripple <amount>",
  body :=
    [ when (does you (cast thisSpell))
        (may (reveal (.slice .top amount (libraryOf .you)))
          (some
            (sequentially
              [ castWithoutPaying
                  (.someOf (.counted anyNumber) (some (.shares .cardName .share this_))
                    (those .card))
                  none,
                .move
                  (allOf
                    (and [isCard, was (reveal which) thisWay, not (was (cast which) thisWay)]))
                  .anyLocation onBottom .arrival [.inOrder .anyOrder] ]))) ]
}
