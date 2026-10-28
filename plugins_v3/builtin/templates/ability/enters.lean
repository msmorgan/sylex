/-- "<subject> enters <riders>", a static ability: as `subject` would enter, it still enters,
with `riders` ("this land enters tapped", "creatures you control enter with an additional +1/+1
counter on them") [CR#603.6d,614.1c,614.1d]. -/
-- [CR#603.6d,614.1c,614.1d]
template enters (subject : NounPhrase) (riders : List MoveRider) : StaticSpec := {
  spelling := "<subject> enters <riders>",
  body :=
    replace
      (would (actor subject) enter)
      (withRiders riders) none .repeatedly none
}
