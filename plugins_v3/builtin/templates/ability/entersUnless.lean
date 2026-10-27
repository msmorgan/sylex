/-- "<subject> enters <riders> unless <condition>": the condition is checked as `subject` would
enter, so the replacement carries it, not the static ability; while it holds, `subject` enters
unmodified [CR#614.1d,614.12]. -/
-- [CR#614.1d,614.12]
template entersUnless (subject : NounPhrase) (riders : List MoveRider)
    (condition : Condition) : StaticSpec := {
  spelling := "<subject> enters <riders> unless <condition>",
  body :=
    Semantics.Templates.StaticSpec.replace
      (Semantics.Templates.WouldClause.would (Semantics.Templates.Subject.actor subject) Semantics.Templates.Action.enter)
      (Semantics.Templates.Replacement.doIf (Semantics.Templates.Condition.not condition) (Semantics.Templates.Replacement.withRiders riders))
      Option.none Semantics.ReplUse.repeatedly Option.none
}
