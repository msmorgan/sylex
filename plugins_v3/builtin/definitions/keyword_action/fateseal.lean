/-- "fateseal N" (`Definition`, [CR#701.29a]): the top N cards of an opponent's library, an
opponent of the performer. -/
action fateseal (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "fateseal", frames := [Amount] }] },
  spelling := "fateseal <amount>",
  body :=
    Semantics.Templates.Action.lookAndSortSlice
      (Semantics.Templates.NounPhrase.slice Semantics.PileEnd.top amount
        (Semantics.Templates.Location.libraryOf
          (Semantics.Templates.NounPhrase.a (Semantics.Templates.Predicate.isOpponentOf Semantics.Templates.NounPhrase.actor))))
      Semantics.Templates.Location.onBottom [Semantics.Templates.MoveRider.inOrder Semantics.Arrangement.anyOrder]
}
