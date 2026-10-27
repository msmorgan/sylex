template thrivingLand (name : String) (own : Color) : Card := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := singleFacedCard
  { name, types := [.land],
    text :=
      [ static (entersTapped thisLand),
        static (entersChoosingFrom thisLand .color (colorOtherThan own)),
        activated tapSymbol (addMana 1 (ofTheChosenColor (some [.of own]))) ] }
}
