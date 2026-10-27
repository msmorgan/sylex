keyword daybound : Ability := {
  lexical := { lemma := "daybound" },
  spelling := "daybound",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.replace
          (Semantics.Templates.WouldClause.would (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisPermanent)
            Semantics.Templates.Action.enter)
          (Semantics.Templates.Replacement.doIf (Semantics.Templates.Condition.gameIs Semantics.Definitions.night)
            (Semantics.Templates.Replacement.withRiders [Semantics.Templates.MoveRider.moveRiderEntersTransformed]))),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.asEvent (Semantics.Templates.GameEvent.gameBecomes Semantics.Definitions.night)
          (Semantics.Templates.Action.doIf
            (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisPermanent
              (Semantics.Templates.Predicate.currentFace Semantics.CardFaceSide.front))
            (Semantics.Definitions.transform Semantics.Templates.NounPhrase.thisPermanent))),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.cant Semantics.Templates.Subject.any
          (Semantics.Definitions.transform Semantics.Templates.NounPhrase.thisPermanent) Option.none Option.none
          (Option.some (Semantics.Templates.DeonticExcept.deonticExceptByKeyword Semantics.Definitions.daybound.label)))]
}
