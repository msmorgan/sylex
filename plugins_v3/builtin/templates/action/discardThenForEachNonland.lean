/-- "discard <discarded>, then <step> for each nonland card discarded this way" (`Template` using
moved-parameter readback, [CR#701.50a,400.7,400.7j]). -/
template discardThenForEachNonland (discarded : NounPhrase) (step : Action) :
    Action := {
  spelling := "discard <discarded>, then <step> for each nonland card discarded this way",
  body :=
    sequentially
      [discard discarded,
        doForEach
          (allOf
            (and
              [non isLand, isCard,
                did (any) (discard which) (within := some thisWay)]))
          step]
}
