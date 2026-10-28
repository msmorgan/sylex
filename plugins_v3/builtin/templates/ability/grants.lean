/-- What one keyword counter confers [CR#122.1b]: its bearer gains the keyword [CR#613.1f], named
by its declaration's name (`grants (.named "flying")`). -/
-- [CR#122.1b]
-- [CR#613.1f]
template grants (keyword : KeywordLabel) : Conferral := {
  spelling := "<keyword>",
  body :=
    .property
      (characteristicChange this_
        [characteristicEditAddedAbilities [.keyword keyword [] []]])
}
