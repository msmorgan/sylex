/-- "<player>'s speed" [CR#702.179a]. -/
-- [CR#702.179a]
primitive speedOf (player : NounPhrase) : Amount := {
  spelling := "<player>'s speed",
  body :=
    Semantics.Primitives.Amount.statOf (Semantics.ProjAxis.playerStat Semantics.PlayerStat.speed) player
}
