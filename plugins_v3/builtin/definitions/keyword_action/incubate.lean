/-- "<performer> incubates N" (`Definition`, [CR#701.53a,701.53b]). -/
action incubate (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "incubate", frames := [Amount] }] },
  spelling := "incubate <amount>",
  body :=
    Semantics.Definitions.create 1
      (Semantics.Templates.TokenSpec.tokenSpecDoubleFaced
        {
          characteristics :=
            { types := [Semantics.CardType.artifact], subtypes := [Semantics.Definitions.incubator],
              text :=
                [Semantics.Templates.Ability.activated (Semantics.Templates.Cost.mana [Semantics.Templates.ManaSymbol.generic 2])
                    (Semantics.Definitions.transform Semantics.Templates.NounPhrase.this_)] } }
        {
          characteristics :=
            { name := Option.some "Phyrexian Token", types := [Semantics.CardType.artifact, Semantics.CardType.creature],
              subtypes := [Semantics.Definitions.phyrexian], power := Option.some 0, toughness := Option.some 0 } })
      [Semantics.Templates.MoveRider.withCounters amount (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
          Semantics.EntryCounterMark.fresh]
}
