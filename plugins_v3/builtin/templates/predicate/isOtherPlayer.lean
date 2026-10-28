/-- "another player", "other player": a bare "other", whose Contrast Anchor is "you" unless the
text mentions a player first ("Choose a player. Each other player…"). -/
template isOtherPlayer : Predicate := {
  spelling := "another player",
  body :=
    and [isPlayer, isOther]
}
