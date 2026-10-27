/- The licensed phrasings of gift: its templating is not normative ("Gift a card", "Gift a
tapped Fish"), so each is a Template over `gift` [CR#702.174a,702.174d..702.174i]. -/

template giftACard : Ability := {
  spelling := "gift a card",
  body :=
    gift (draw 1)
}
