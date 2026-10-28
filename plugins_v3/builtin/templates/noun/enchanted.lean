/-- "enchanted <w>" -/
template enchanted (w : NounWord) : NounPhrase := {
  spelling := "enchanted <w>",
  body := attachHost .enchanted w
}
