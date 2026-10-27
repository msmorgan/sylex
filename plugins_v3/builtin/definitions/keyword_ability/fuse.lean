/-- "Fuse": a player who casts this split card from their hand may cast both of its halves, as
one fused split spell [CR#702.102a], whose total cost includes each half's mana cost
[CR#702.102c]. -/
keyword fuse : Ability := {
  lexical := { lemma := "fuse" },
  spelling := "fuse",
  body :=
    [static (can you (cast thisCard (from_ := yourHand) (payment := fused)))]
}
