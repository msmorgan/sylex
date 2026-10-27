action populate : Action := {
  lexical := { verbs := [{ lemma := "populate", frames := [Intransitive] }] },
  spelling := "populate",
  body :=
    Semantics.Templates.Action.sequentially
      [Semantics.Templates.Action.choose
          (Semantics.Templates.NounPhrase.a
            (Semantics.Templates.Predicate.and
              [Semantics.Templates.Predicate.isToken, Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.isPermanent,
                Semantics.Templates.Predicate.actorControls])),
        Semantics.Definitions.create 1 (Semantics.Templates.TokenSpec.copyOf Semantics.Templates.NounPhrase.thatToken)]
}
