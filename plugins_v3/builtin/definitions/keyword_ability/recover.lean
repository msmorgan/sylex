/-- "Recover [cost]" means "When a creature is put into your graveyard from the battlefield, you may
pay [cost]. If you do, return this card from your graveyard to your hand. Otherwise, exile this
card." [CR#702.59a] -/
keyword recover (cost : Cost) : Ability := {
  lexical := { lemma := "recover", parameters := [[Cost]] },
  spelling := "recover <cost>",
  body :=
    [when
        (does any
          (move (a isCreature) battlefield
            (some yourGraveyard) .departure []))
        (may (pay cost once)
          (some (returnToHand this_ yourGraveyard))
          (some (exile this_)))]
}
