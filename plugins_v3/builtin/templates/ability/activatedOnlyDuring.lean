/-- "[cost]: <action>. Activate only <timing>." -/
template activatedOnlyDuring (cost : Cost) (action : Action) (timing : Timing) :
    Ability := {
  spelling := "<cost>: <action>. Activate only <timing>.",
  body := activated cost action (some timing) none none none
}
