/-- "<subject> enters <riders>", a static ability: as `subject` would enter, it still enters,
with `riders` ("this land enters tapped", "creatures you control enter with an additional +1/+1
counter on them") [CR#603.6d,614.1c,614.1d]. -/
-- [CR#603.6d,614.1c,614.1d]
template enters (subject : NounPhrase) (riders : List MoveRider) : StaticSpec := {
  spelling := "<subject> enters <riders>",
  body :=
    Semantics.Templates.StaticSpec.replace
      (Semantics.Templates.WouldClause.would (Semantics.Templates.Subject.actor subject) Semantics.Templates.Action.enter)
      (Semantics.Templates.Replacement.withRiders riders) Option.none Semantics.ReplUse.repeatedly Option.none
}
