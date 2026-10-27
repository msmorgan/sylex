/-- "<performer> waterbends <cost>" (`Definition`, [CR#701.67a,701.67b]). -/
action waterbend (cost : Cost) : Action := {
  lexical := { verbs := [{ lemma := "waterbend", past := "waterbent", frames := [Cost, Intransitive] }] },
  spelling := "waterbend <cost>",
  body :=
    .pay
        (.withSubstitute cost
          (substitute (tap (a (and [isUntapped, or [isArtifact, isCreature], actorControls])))
            [.generic]))
        .once .paid none none
}
