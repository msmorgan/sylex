/-- "Disturb [cost]": "You may cast this card transformed from your graveyard by paying [cost]
rather than its mana cost." [CR#702.146a] Cast transformed, the spell has its back face up
[CR#712.8c], and the permanent it becomes enters with its back face up [CR#702.146b]. -/
keyword disturb (cost : Cost) : Ability := {
  lexical := { lemma := "disturb", parameters := [[Cost]] },
  spelling := "disturb <cost>",
  body :=
    [ static
        (can you
          (cast this_ (from_ := yourGraveyard) (payment := transformedPaying cost))) ]
}
