/-- "Proliferate" with its reminder text (`Definition`, [CR#701.34a]). -/
action proliferate : Action := {
  lexical := { verbs := [{ lemma := "proliferate", frames := [Intransitive] }] },
  spelling := "proliferate",
  body :=
    sequentially
      [choose (counted anyNumber
            (and
              [andOr [isPermanent, isPlayer],
                hasCounter none])) openly none none,
        doForEach (those join)
          (putCounters 1 (sameKindAs it)
            it)]
}
