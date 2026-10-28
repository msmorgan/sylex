/-- "Gift a Treasure" means the effect is "The chosen player creates a Treasure
token." [CR#702.174h] A Treasure token is a colorless Treasure artifact
token with "{T}, Sacrifice this token: Add one mana of any color."
[CR#111.10a] -/
template giftATreasure : Ability := {
  spelling := "gift a Treasure",
  body :=
    gift (create 1 treasureToken)
}
