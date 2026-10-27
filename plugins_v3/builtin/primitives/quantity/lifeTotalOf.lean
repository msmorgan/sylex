primitive lifeTotalOf (player : NounPhrase) : Amount := {
  spelling := "<player>'s life total",
  body :=
    Semantics.Primitives.Amount.statOf (Semantics.ProjAxis.playerStat Semantics.PlayerStat.lifeTotal) player
}
