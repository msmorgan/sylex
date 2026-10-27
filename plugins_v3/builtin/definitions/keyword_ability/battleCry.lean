keyword battleCry : Ability := {
  lexical := { lemma := "battle cry" },
  spelling := "battle cry",
  body :=
    [Semantics.Templates.Ability.whenever
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisCreature)
          (Semantics.Templates.Action.attack Option.none))
        (Semantics.Templates.Action.establishThroughout
          (Semantics.Templates.StaticSpec.getsPt
            (Semantics.Templates.NounPhrase.each
              (Semantics.Templates.Predicate.and
                [Semantics.Templates.Predicate.isCreature, Semantics.Templates.Predicate.isAttacking, Semantics.Templates.Predicate.isOther]))
            (Semantics.Templates.up 1) (Semantics.Templates.up 0))
          Semantics.Templates.Duration.untilEndOfTurn)]
}
