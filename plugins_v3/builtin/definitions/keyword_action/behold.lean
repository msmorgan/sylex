action behold (quality : Predicate) : Action := {
  lexical := { verbs := [{ lemma := "behold", past := "beheld", frames := [Monotransitive] }] },
  spelling := "behold <quality>",
  body :=
    Semantics.Templates.Action.chooseOne
      (Semantics.Templates.Action.expose Semantics.ExposeVerb.reveal
        (Semantics.Templates.Exposed.cards
          (Semantics.Templates.NounPhrase.a
            (Semantics.Templates.Predicate.and
              [Semantics.Templates.Predicate.isCard, quality,
                Semantics.Templates.Predicate.in_ (Semantics.Templates.Location.handOf Semantics.Templates.NounPhrase.actor)]))))
      (Semantics.Templates.Action.choose
        (Semantics.Templates.NounPhrase.a
          (Semantics.Templates.Predicate.and
            [quality, Semantics.Templates.Predicate.isPermanent, Semantics.Templates.Predicate.actorControls])))
}
