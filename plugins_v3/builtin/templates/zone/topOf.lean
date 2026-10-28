/-- "the top of <loc>": the first place from its top [CR#401.7]. -/
-- [CR#401.7]
template topOf (loc : Location) : Location := {
  spelling := "the top of <loc>",
  body := fromEnd .top (.nth 1) loc
}
