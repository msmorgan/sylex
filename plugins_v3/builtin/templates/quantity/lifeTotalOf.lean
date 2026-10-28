template lifeTotalOf (player : NounPhrase) : Amount := {
  spelling := "<player>'s life total",
  body := statOf (.playerStat .lifeTotal) player
}
