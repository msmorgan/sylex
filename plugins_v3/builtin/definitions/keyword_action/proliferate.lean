/-- "Proliferate" with its reminder text (`Definition`, [CR#701.34a]). -/
action proliferate : Action := {
  lexical := { verbs := [{ lemma := "proliferate", frames := [Intransitive] }] },
  spelling := "proliferate",
  body :=
    sequentially
      [ .choose none (counted anyNumber (and [andOr [isPermanent, isPlayer], .hasCounters none]))
          .openly none,
        .doForEach (those .join) (putCounters 1 (.sameAs it) it) ]
}
