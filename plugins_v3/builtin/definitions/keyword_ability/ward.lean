keyword ward (cost : Cost) : Ability := {
  lexical := { lemma := "ward", parameters := [[Cost], [DashNonManaCost]] },
  -- TODO(spelling): ward [mana cost] / ward—[verbal cost]; separator and terminal period depend
  -- on the Cost structure.
  body :=
    [Semantics.Templates.Ability.whenever
        (Semantics.Templates.GameEvent.does
          (Semantics.Templates.Subject.by_
            (Semantics.Templates.NounPhrase.a
              (Semantics.Templates.Predicate.and
                [Semantics.Templates.Predicate.in_ Semantics.Templates.Location.stack, Semantics.Templates.Predicate.opponentsControl])))
          (Semantics.Templates.Action.target Semantics.Templates.NounPhrase.thisPermanent))
        (Semantics.Templates.Action.instruct (Semantics.Templates.NounPhrase.controllerOf (Semantics.Templates.NounPhrase.that Semantics.NounWord.stack))
          (Semantics.Templates.Action.mayOrElse (Semantics.Templates.Action.pay cost Semantics.PayTimes.once)
            (Semantics.Templates.Action.instruct Semantics.Templates.NounPhrase.you
              (Semantics.Definitions.counter (Semantics.Templates.NounPhrase.that Semantics.NounWord.stack)))))]
}
