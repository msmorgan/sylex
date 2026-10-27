/-- "only as a sorcery": while you have priority, during your main phase, while the stack is
empty [CR#307.5]. Its negation, "any time a sorcery couldn't have been cast", therefore also
holds without priority [CR#307.5a]. -/
-- [CR#307.5]
-- [CR#307.5a]
template asSorcery : Timing := {
  spelling := "only as a sorcery",
  body :=
    Semantics.Templates.Timing.and
      [Semantics.Templates.Timing.whileTrue (Semantics.Templates.Condition.conditionHasPriority Semantics.Templates.NounPhrase.you),
        Semantics.Templates.Timing.timingInPart Semantics.TurnPart.mainPhase (Option.some Semantics.Templates.NounPhrase.you)
          Semantics.Templates.TurnRef.turnRefCurrent,
        Semantics.Templates.Timing.whileTrue
          (Semantics.Templates.Condition.not (Semantics.Templates.Condition.exists_ (Semantics.Templates.Predicate.in_ Semantics.Templates.Location.stack)))]
}
