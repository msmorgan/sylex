/-- "triple <subject>'s toughness": it gets +0/+X, X twice its toughness (`Definition`,
[CR#701.11b,701.11c]). -/
action tripleToughness (subject : NounPhrase) (duration : Option Duration := none) :
    Action := {
  -- TODO(spelling): triple <subject>'s toughness [<duration>]; string substitution cannot form
  -- possessives or omit an absent duration.
  lexical := { verbs := [{ lemma := "triple", frames := [Monotransitive] }] },
  deed := "triple",
  body :=
    get subject (.up 0)
        (.up
          (times 2
            (toughnessOf subject)))
        duration
}
