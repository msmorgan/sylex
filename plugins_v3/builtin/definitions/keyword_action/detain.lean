/-- "detain <subject>": until the next turn of the controller of the detaining spell or ability,
"your next turn" [CR#701.35a,109.5], which is not its source's controller once the source has
changed control [CR#113.8]. -/
action detain (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "detain", frames := [Monotransitive] }] },
  spelling := "detain <subject>",
  body :=
    Semantics.Templates.Action.establishThroughout
      (Semantics.Templates.StaticSpec.staticSpecConjunction Option.none
        [Semantics.Templates.StaticSpec.cant (Semantics.Templates.Subject.actor subject) (Semantics.Templates.Action.attack Option.none),
          Semantics.Templates.StaticSpec.cant (Semantics.Templates.Subject.actor subject) (Semantics.Templates.Action.block Semantics.Templates.NounPhrase.any),
          Semantics.Templates.StaticSpec.cant Semantics.Templates.Subject.any
            (Semantics.Definitions.activate
              (Semantics.Templates.NounPhrase.allOf
                (Semantics.Templates.Predicate.and
                  [Semantics.Templates.Predicate.abilityHead Semantics.AbilityClass.anyActivated,
                    Semantics.Templates.Predicate.abilityOf subject])))])
      Semantics.Templates.Duration.untilYourNextTurn
}
