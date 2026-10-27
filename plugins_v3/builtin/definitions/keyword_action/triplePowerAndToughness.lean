/-- "triple <subject>'s power and toughness" (`Definition`, [CR#701.11b,701.11c]). -/
action triplePowerAndToughness (subject : NounPhrase)
    (duration : Option Duration := none) : Action := {
  -- TODO(spelling): triple <subject>'s power and toughness [<duration>]; string substitution cannot
  -- form possessives or omit an absent duration.
  lexical := { verbs := [{ lemma := "triple", frames := [Monotransitive] }] },
  deed := "triple",
  body :=
    get subject
        (.up
          (times 2
            (powerOf subject)))
        (.up
          (times 2
            (toughnessOf subject)))
        duration
}
