/-- "roll <count> <sides>-sided dice [and watch for <watch>]": as a pattern, "roll a die" is the
default `1` and "roll one or more dice" `anyAmount`. -/
primitive rollDice (count : Amount := 1)
    (sides : DieSides := .sides 6) (watch : RollWatch := anyResult) : Action := {
  -- TODO(spelling): Context-dependent wording: roll <count> <sides>-sided dice [and watch for <watch>]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Action.rollDice count sides watch
}
