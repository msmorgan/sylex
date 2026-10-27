/-- "Encore [cost]": "[Cost], Exile this card from your graveyard: For each opponent, create a
token that's a copy of this card that attacks that opponent this turn if able. The tokens gain
haste. Sacrifice them at the beginning of the next end step. Activate only as a sorcery."
[CR#702.141a] -/
keyword encore (cost : Cost) : Ability := {
  lexical := { lemma := "encore", parameters := [[Cost]] },
  spelling := "encore <cost>",
  body :=
    [ activatedOnlyDuring
        (.compound [cost, .perform (exileFrom thisCard yourGraveyard)])
        (sequentially
          [ .doForEach eachOpponent
              (sequentially
                [ create 1 (copyOf this_),
                  establishThroughout (must thatToken (.attack (some (that player)))) thisTurn ]),
            gain thoseTokens (keyword haste),
            atNext .endStep (sacrifice thoseTokens) ])
        asSorcery ]
}
