/-- "Proliferate" with its reminder text (`Definition`, [CR#701.34a]). -/
action proliferate : Action := {
  lexical := { verbs := [{ lemma := "proliferate", frames := [Intransitive] }] },
  spelling := "proliferate",
  body :=
    Semantics.Templates.Action.sequentially
      [Semantics.Templates.Action.actionChoose Option.none
          (Semantics.Templates.NounPhrase.counted Semantics.Templates.Quantity.anyNumber
            (Semantics.Templates.Predicate.and
              [Semantics.Templates.Predicate.andOr [Semantics.Templates.Predicate.isPermanent, Semantics.Templates.Predicate.isPlayer],
                Semantics.Templates.Predicate.hasCounter Option.none]))
          Semantics.Disclosure.openly Option.none,
        Semantics.Templates.Action.doForEach (Semantics.Templates.NounPhrase.those Semantics.NounWord.join)
          (Semantics.Templates.Action.putCounters 1 (Semantics.Templates.CounterKindSource.sameKindAs Semantics.Templates.NounPhrase.it)
            Semantics.Templates.NounPhrase.it)]
}
