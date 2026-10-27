template sacrificedWhenItLeaves (source : NounPhrase) : Action := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := delay (does source (leave battlefield)) (instruct (controllerOf (that creature)) (sacrifice it))
}
