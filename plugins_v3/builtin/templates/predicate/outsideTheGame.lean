/-- "outside the game": in none of the game's zones. Outside the game is not a zone
[CR#400.11], so it is no `Location` but the complement of all seven [CR#400.1]: a sideboard's
cards are there [CR#400.11a], and so is a Lesson that learn reveals [CR#701.48a]. -/
-- [CR#400.11]
-- [CR#400.1]
-- [CR#400.11a]
-- [CR#701.48a]
template outsideTheGame : Predicate := {
  spelling := "outside the game",
  body :=
    Semantics.Templates.Predicate.not
      (Semantics.Templates.Predicate.in_
        (Semantics.Templates.Location.or
          [Semantics.Templates.Location.library, Semantics.Templates.Location.hand, Semantics.Templates.Location.battlefield,
            Semantics.Templates.Location.graveyard, Semantics.Templates.Location.stack, Semantics.Templates.Location.exileZone,
            Semantics.Templates.Location.commandZone]))
}
