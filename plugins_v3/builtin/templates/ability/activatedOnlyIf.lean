/-- "[cost]: <action>. Activate only if <guard>." -/
template activatedOnlyIf (cost : Cost) (action : Action) (guard : Condition) :
    Ability := {
  spelling := "<cost>: <action>. Activate only if <guard>.",
  body := activated cost action none none (some guard) none
}
