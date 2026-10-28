/-- "Gift a tapped Fish" means the effect is "The chosen player creates a
tapped 1/1 blue Fish creature token." [CR#702.174f] -/
template giftATappedFish : Ability := {
  spelling := "gift a tapped Fish",
  body :=
    gift (createTappedToken (creatureToken 1 1 [.blue] [fish]))
}
