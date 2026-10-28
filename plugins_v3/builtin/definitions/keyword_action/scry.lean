/-- "scry N" (`Definition`, [CR#701.22a]): the performer's own library. -/
action scry (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "scry", frames := [Amount, Intransitive, Monotransitive] }] },
  spelling := "scry <amount>",
  body :=
    lookAndSortSlice
      (slice Semantics.PileEnd.top amount (libraryOf actor))
      onBottom [inOrder Semantics.Arrangement.anyOrder]
}
