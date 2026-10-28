/-- "Proliferate" with its reminder text (`Definition`, [CR#701.34a]). -/
action proliferate : Action := {
  lexical := { verbs := [{ lemma := "proliferate", frames := [Intransitive] }] },
  spelling := "proliferate",
  body :=
    sequentially
      [actionChoose none
          (counted anyNumber
            (and
              [andOr [isPermanent, isPlayer],
                hasCounter none]))
          Semantics.Disclosure.openly none,
        doForEach (those Semantics.NounWord.join)
          (putCounters 1 (sameKindAs it)
            it)]
}
