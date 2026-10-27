/-- "discard <discarded>, then <step> for each nonland card discarded this way" (`Template` using
moved-parameter readback, [CR#701.50a,400.7,400.7j]). -/
template discardThenForEachNonland (discarded : NounPhrase) (step : Action) :
    Action := {
  spelling := "discard <discarded>, then <step> for each nonland card discarded this way",
  body :=
    Semantics.Templates.Action.sequentially
      [Semantics.Definitions.discard discarded,
        Semantics.Templates.Action.doForEach
          (Semantics.Templates.NounPhrase.allOf
            (Semantics.Templates.Predicate.and
              [Semantics.Templates.Predicate.non Semantics.Templates.Predicate.isLand, Semantics.Templates.Predicate.isCard,
                Semantics.Templates.Predicate.predicateDid
                  { subj := Semantics.Templates.Subject.any, action := Semantics.Definitions.discard Semantics.Templates.NounPhrase.which,
                    within := Option.some Semantics.Templates.LookbackSpan.thisWay }]))
          step]
}
