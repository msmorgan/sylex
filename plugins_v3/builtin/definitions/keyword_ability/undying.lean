keyword undying : Ability := {
  lexical := { lemma := "undying" },
  spelling := "undying",
  body :=
    [Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisPermanent) Semantics.Templates.Action.die)
        (Semantics.Templates.Action.returnToBattlefieldWithCounters Semantics.Templates.NounPhrase.thisPermanent
          (Semantics.Templates.NounPhrase.ownerOf Semantics.Templates.NounPhrase.thisPermanent) 1 Semantics.Definitions.p1p1Counter)
        [] [] Option.none Option.none Option.none
        (Option.some
          (Semantics.Templates.Condition.not
            (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisPermanent
              (Semantics.Templates.Predicate.hasCounter (Option.some Semantics.Definitions.p1p1Counter)))))]
}
