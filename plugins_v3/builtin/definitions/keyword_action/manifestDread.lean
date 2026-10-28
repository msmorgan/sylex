/-- "<performer> manifests dread" (`Definition`, [CR#701.62a]). -/
action manifestDread : Action := {
  lexical := { verbs := [{ lemma := "manifest dread", third_singular := "manifests dread", past := "manifested dread", gerund := "manifesting dread", frames := [Intransitive] }] },
  spelling := "manifest dread",
  body :=
    sequentially
      [expose Semantics.ExposeVerb.lookAt
          (cards
            (slice Semantics.PileEnd.top 2
              (libraryOf actor))),
        manifest
          (someOf (exactly 1) them),
        putInto theRest graveyard]
}
