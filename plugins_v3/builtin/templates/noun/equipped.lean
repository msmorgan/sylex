/-- "equipped <w>" -/
template equipped (w : NounWord) : NounPhrase := {
  spelling := "equipped <w>",
  body := attachHost .equipped w
}
