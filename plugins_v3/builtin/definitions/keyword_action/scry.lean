/-- "scry N" (`Definition`, [CR#701.22a]): the performer's own library. -/
action scry (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "scry", frames := [Amount, Intransitive, Monotransitive] }] },
  spelling := "scry <amount>",
  body :=
    Semantics.Templates.Action.lookAndSortSlice
      (Semantics.Templates.NounPhrase.slice Semantics.PileEnd.top amount (Semantics.Templates.Location.libraryOf Semantics.Templates.NounPhrase.actor))
      Semantics.Templates.Location.onBottom [Semantics.Templates.MoveRider.inOrder Semantics.Arrangement.anyOrder]
}
