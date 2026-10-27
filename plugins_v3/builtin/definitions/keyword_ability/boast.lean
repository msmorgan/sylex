keyword boast (ability : Ability) : Ability := {
  lexical := {
    lemma := "boast",
    parameters := [[DashAbility]],
    verbs := [{ lemma := "boast", frames := [Intransitive] }]
  },
  spelling := "boast — <ability>",
  body :=
    [Semantics.Templates.Ability.qualified ability Option.none (Option.some Semantics.Templates.UsageLimit.oncePerTurn)
        (Option.some
          (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisCreature
            (Semantics.Templates.Predicate.did (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.which)
              (Semantics.Templates.Action.attack Option.none)
              (Option.some (Semantics.Templates.LookbackSpan.within Semantics.Templates.Duration.soFarThisTurn)))))
        Option.none]
}
