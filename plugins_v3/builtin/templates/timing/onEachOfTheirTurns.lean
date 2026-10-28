/-- "on each of their turns": each of the turns of the player the deontic subject names, "their"
the singular pronoun resolved distributively ("Each player may play an additional land on each
of their turns"). -/
template onEachOfTheirTurns : Timing := {
  spelling := "on each of their turns",
  body :=
    inPart .turn
      (some (pro (.word .player) .one))
      (eachTime none)
}
