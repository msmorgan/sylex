/-- "<performer> incubates N" (`Definition`, [CR#701.53a,701.53b]). -/
action incubate (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "incubate", frames := [Amount] }] },
  spelling := "incubate <amount>",
  body :=
    create 1
      (doubleFaced
        {
          characteristics :=
            { types := [Semantics.CardType.artifact], subtypes := [incubator],
              text :=
                [activated (mana [generic 2])
                    (transform this_)] } }
        {
          characteristics :=
            { name := some "Phyrexian Token", types := [Semantics.CardType.artifact, Semantics.CardType.creature],
              subtypes := [phyrexian], power := some 0, toughness := some 0 } })
      [withCounters amount (printedKind p1p1Counter)
          fresh]
}
