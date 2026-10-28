/-- "Afterlife N" means "When this permanent is put into a graveyard from the battlefield, create N
1/1 white and black Spirit creature tokens with flying." [CR#702.135a] -/
keyword afterlife (amount : Amount) : Ability := {
  lexical := { lemma := "afterlife", parameters := [[Amount]] },
  spelling := "afterlife <amount>",
  body :=
    [ when (does thisPermanent die)
        (create amount (creatureToken 1 1 [.white, .black] [spirit] [flying])) ]
}
