/-- "adapt N" with its reminder text (`Definition`, [CR#701.46a]): an instruction about "this
permanent" (`DeedPermanent.isAbout`), the Source unless a handoff names another ("target creature
adapts"), that the ability's controller follows [CR#608.2c]; the body reads that permanent as
`.actor`. -/
action adapt (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "adapt", frames := [Amount, Intransitive] }] },
  permanent := isAbout,
  spelling := "adapt <amount>",
  body :=
    Semantics.Templates.Action.doIf
      (Semantics.Templates.Condition.not
        (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.actor
          (Semantics.Templates.Predicate.hasCounter (Option.some Semantics.Definitions.p1p1Counter))))
      (Semantics.Templates.Action.putCounters amount (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
        Semantics.Templates.NounPhrase.actor)
}
