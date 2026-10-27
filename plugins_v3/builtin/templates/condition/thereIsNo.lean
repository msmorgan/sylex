/-- "if there is no <designation>" ("if there is no monarch"): nothing holds it. -/
template thereIsNo (designation : DesignationLabel) : Condition := {
  spelling := "if there is no <designation>",
  body :=
    Semantics.Templates.Condition.not
      (Semantics.Templates.Condition.thereAre
        (Semantics.Templates.NounPhrase.described (Semantics.Templates.DetPhrase.detPhraseA Semantics.ChoiceMode.unmarked)
          (Semantics.Templates.Predicate.hasDesignation designation Option.none)))
}
