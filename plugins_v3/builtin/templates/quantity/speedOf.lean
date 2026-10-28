/-- "<player>'s speed" [CR#702.179a]. -/
-- [CR#702.179a]
template speedOf (player : NounPhrase) : Amount := {
  spelling := "<player>'s speed",
  body := statOf (.playerStat speed) player
}
