/-- "that <word>", e.g. `that .player`. -/
template that (w : NounWord) : NounPhrase := {
  spelling := "that <w>",
  body := pro (.word w) .one
}
