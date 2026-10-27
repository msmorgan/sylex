keyword decayed : Ability := {
  lexical := { lemma := "decayed" },
  spelling := "decayed",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.cant (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisCreature)
          (Semantics.Templates.Action.block Semantics.Templates.NounPhrase.any)),
      Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisCreature)
          (Semantics.Templates.Action.attack Option.none))
        (Semantics.Templates.Action.atNext Semantics.TurnPart.endOfCombat
          (Semantics.Definitions.sacrifice Semantics.Templates.NounPhrase.thisPermanent))]
}
