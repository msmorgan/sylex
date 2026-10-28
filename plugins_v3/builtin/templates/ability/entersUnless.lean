/-- "<subject> enters <riders> unless <condition>": the condition is checked as `subject` would
enter, so the replacement carries it, not the static ability; while it holds, `subject` enters
unmodified [CR#614.1d,614.12]. -/
-- [CR#614.1d,614.12]
template entersUnless (subject : NounPhrase) (riders : List MoveRider)
    (condition : Condition) : StaticSpec := {
  spelling := "<subject> enters <riders> unless <condition>",
  body :=
    replace
      (would (actor subject) enter)
      (doIf (not condition) (withRiders riders))
      none Semantics.ReplUse.repeatedly none
}
