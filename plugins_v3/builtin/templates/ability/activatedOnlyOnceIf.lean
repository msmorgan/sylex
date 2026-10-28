/-- "[cost]: <action>. Activate only once <limit> and only if <guard>." -/
template activatedOnlyOnceIf (cost : Cost) (action : Action) (limit : UsageLimit)
    (guard : Condition) : Ability := {
  spelling := "<cost>: <action>. Activate only once <limit> and only if <guard>.",
  body := activated cost action none (some limit) (some guard) none
}
