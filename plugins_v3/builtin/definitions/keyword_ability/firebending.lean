keyword firebending (amount : Amount) : Ability := {
  lexical := {
    lemma := "firebending",
    parameters := [[Amount]],
    verbs := [{ lemma := "firebend", past := "firebent", frames := [Intransitive] }]
  },
  spelling := "firebending <amount>",
  body :=
    [Semantics.Templates.Ability.whenever
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisCreature)
          (Semantics.Templates.Action.attack Option.none))
        (Semantics.Templates.Action.sequentially
          [Semantics.Templates.Action.addMana amount
              (Semantics.Templates.ProducedMana.manaOf [[Semantics.ColorOrColorless.of Semantics.Color.red]]) [],
            Semantics.Templates.Action.establishThroughout
              (Semantics.Templates.StaticSpec.manaRetention Semantics.Templates.NounPhrase.you Semantics.Templates.ManaHeld.thisMana)
              Semantics.Templates.Duration.untilEndOfCombat])]
}
