/-- "monstrosity N" with its reminder text (`Definition`, [CR#701.37a]): an instruction about
"this permanent" (`DeedPermanent.isAbout`), the Source unless a handoff names another, that the
ability's controller follows [CR#608.2c,701.37c]; the body reads that permanent as `.actor`. -/
action monstrosity (amount : Amount) : Action := {
  lexical := { nouns := [{ lemma := "monstrosity", frames := [(Complement Amount)], parameters := [[Head, (Slot Amount)]] }] },
  permanent := isAbout,
  spelling := "monstrosity <amount>",
  body :=
    Semantics.Templates.Action.doIf
      (Semantics.Templates.Condition.not
        (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.actor
          (Semantics.Templates.Predicate.hasDesignation Semantics.Definitions.monstrous)))
      (Semantics.Templates.Action.sequentially
        [Semantics.Templates.Action.putCounters amount (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.p1p1Counter)
            Semantics.Templates.NounPhrase.actor,
          Semantics.Templates.Action.gainDesignation Semantics.Templates.NounPhrase.actor Semantics.Definitions.monstrous])
}
