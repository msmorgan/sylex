/-- "<np> and/or <np>", as "a card named Magnifying Glass and/or a card named Thinking Cap". -/
template andOr (phrases : List NounPhrase := []) : NounPhrase := {
  body := coord .andOr phrases
}
