action forage : Action := {
  lexical := { verbs := [{ lemma := "forage", frames := [Intransitive] }] },
  spelling := "forage",
  body :=
    Semantics.Templates.Action.chooseOne
      (Semantics.Definitions.exileFrom
        (Semantics.Templates.NounPhrase.counted (Semantics.Templates.Quantity.exactly 3)
          (Semantics.Templates.Predicate.cardIn (Semantics.Templates.Location.graveyardOf Semantics.Templates.NounPhrase.actor)))
        (Semantics.Templates.Location.graveyardOf Semantics.Templates.NounPhrase.actor))
      (Semantics.Definitions.sacrifice
        (Semantics.Templates.NounPhrase.a
          (Semantics.Templates.Predicate.and
            [Semantics.Templates.Predicate.hasSubtype Semantics.Definitions.food, Semantics.Templates.Predicate.isPermanent,
              Semantics.Templates.Predicate.actorControls])))
}
