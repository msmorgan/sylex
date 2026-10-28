/-- "fateseal N" (`Definition`, [CR#701.29a]): the top N cards of an opponent's library, an
opponent of the performer. -/
action fateseal (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "fateseal", frames := [Amount] }] },
  spelling := "fateseal <amount>",
  body :=
    lookAndSortSlice
      (slice .top amount
        (libraryOf
          (a (isOpponentOf actor))))
      onBottom [inOrder .anyOrder]
}
