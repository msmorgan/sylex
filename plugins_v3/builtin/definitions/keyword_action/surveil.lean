/-- "surveil N" (`Definition`, [CR#701.25a]): the performer's own library. -/
action surveil (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "surveil", past := "surveilled", gerund := "surveilling", frames := [Amount, Intransitive, Monotransitive] }] },
  spelling := "surveil <amount>",
  body :=
    Semantics.Templates.Action.lookAndSortSlice
      (Semantics.Templates.NounPhrase.slice Semantics.PileEnd.top amount (Semantics.Templates.Location.libraryOf Semantics.Templates.NounPhrase.actor))
      Semantics.Templates.Location.graveyard
}
