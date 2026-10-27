keyword nightbound : Ability := {
  lexical := { lemma := "nightbound" },
  spelling := "nightbound",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.asEvent (Semantics.Templates.GameEvent.gameBecomes Semantics.Definitions.day)
          (Semantics.Templates.Action.doIf
            (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisPermanent Semantics.Templates.Predicate.isTransformed)
            (Semantics.Definitions.transform Semantics.Templates.NounPhrase.thisPermanent))),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.cant Semantics.Templates.Subject.any
          (Semantics.Definitions.transform Semantics.Templates.NounPhrase.thisPermanent) Option.none Option.none
          (Option.some (Semantics.Templates.DeonticExcept.deonticExceptByKeyword Semantics.Definitions.nightbound.label)))]
}
