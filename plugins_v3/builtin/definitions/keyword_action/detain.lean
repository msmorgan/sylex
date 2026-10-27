/-- "detain <subject>": until the next turn of the controller of the detaining spell or ability,
"your next turn" [CR#701.35a,109.5], which is not its source's controller once the source has
changed control [CR#113.8]. -/
action detain (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "detain", frames := [Monotransitive] }] },
  spelling := "detain <subject>",
  body :=
    establishThroughout
        (.conjunction none
          [ cant (actor subject) (.attack none),
            cant (actor subject) (.block .any),
            cant .any
              (activate (allOf
                  (and
                    [ .abilityHead .anyActivated,
                      .abilityOf subject ]))) ])
        untilYourNextTurn
}
