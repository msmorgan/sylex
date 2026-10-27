keyword ingest : Ability := {
  lexical := { lemma := "ingest" },
  spelling := "ingest",
  body :=
    [Semantics.Templates.Ability.whenever
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisCreature)
          (Semantics.Templates.Action.dealDamage Semantics.Templates.Amount.anyAmount (Semantics.Templates.NounPhrase.a Semantics.Templates.Predicate.isPlayer)
            (Option.some Semantics.DamageKind.combat)))
        (Semantics.Templates.Action.instruct Semantics.Templates.NounPhrase.they
          (Semantics.Definitions.exile
            (Semantics.Templates.NounPhrase.slice Semantics.PileEnd.top 1 (Semantics.Templates.Location.libraryOf Semantics.Templates.NounPhrase.they))))]
}
