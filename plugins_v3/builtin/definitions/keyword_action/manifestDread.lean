/-- "<performer> manifests dread" (`Definition`, [CR#701.62a]). -/
action manifestDread : Action := {
  lexical := { verbs := [{ lemma := "manifest dread", third_singular := "manifests dread", past := "manifested dread", gerund := "manifesting dread", frames := [Intransitive] }] },
  spelling := "manifest dread",
  body :=
    Semantics.Templates.Action.sequentially
      [Semantics.Templates.Action.expose Semantics.ExposeVerb.lookAt
          (Semantics.Templates.Exposed.cards
            (Semantics.Templates.NounPhrase.slice Semantics.PileEnd.top 2
              (Semantics.Templates.Location.libraryOf Semantics.Templates.NounPhrase.actor))),
        Semantics.Definitions.manifest
          (Semantics.Templates.NounPhrase.someOf (Semantics.Templates.Quantity.exactly 1) Semantics.Templates.NounPhrase.them),
        Semantics.Templates.Action.putInto Semantics.Templates.NounPhrase.theRest Semantics.Templates.Location.graveyard]
}
