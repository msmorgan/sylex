/-- "if there is no <designation>" ("if there is no monarch"): nothing holds it. -/
template thereIsNo (designation : DesignationLabel) : Condition := {
  spelling := "if there is no <designation>",
  body :=
    not
      (thereAre
        (described (detPhraseA Semantics.ChoiceMode.unmarked)
          (hasDesignation designation none)))
}
