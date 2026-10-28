/-- "<subject> wins the game" -/
template winGame : Action := {
  -- TODO(spelling): Context-dependent wording: <subject> wins the game; string substitution cannot express its omitted or inflected parts.
  body := conclude .winGame
}
