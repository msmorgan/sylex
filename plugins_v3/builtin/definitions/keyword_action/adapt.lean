/-- "adapt N" with its reminder text (`Definition`, [CR#701.46a]): an instruction about "this
permanent" (`DeedPermanent.isAbout`), the Source unless a handoff names another ("target creature
adapts"), that the ability's controller follows [CR#608.2c]; the body reads that permanent as
`.actor`. -/
action adapt (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "adapt", frames := [Amount, Intransitive] }] },
  permanent := isAbout,
  spelling := "adapt <amount>",
  body :=
    doIf (not (is .actor (.hasCounters p1p1Counter)))
        (putCounters amount p1p1Counter .actor)
        none
}
