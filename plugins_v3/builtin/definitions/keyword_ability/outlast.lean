keyword outlast (cost : Cost) : Ability := {
  lexical := { lemma := "outlast", parameters := [[Cost]] },
  spelling := "outlast <cost>",
  body :=
    [Semantics.Templates.Ability.activatedOnlyDuring (Semantics.Templates.Cost.compound [cost, Semantics.Templates.Cost.tapSymbol])
        (Semantics.Templates.Action.putCounters 1 (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
          Semantics.Templates.NounPhrase.thisCreature)
        Semantics.Templates.Timing.asSorcery]
}
