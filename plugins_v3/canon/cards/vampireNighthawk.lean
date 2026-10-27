/- ## The results of damage, on cards

Lifelink adds a result to its damage [CR#702.15b]; wither and infect replace the result the
damage would otherwise have [CR#702.80a,702.90b,702.90c] (`Definitions/Keywords.lean`).
*Soul-Scar Mage* replaces the damage itself, not its result [CR#614.1a]. -/
card vampireNighthawk : Spelled :=
  { name := "Vampire Nighthawk", cost := [generic 1, pip .black, pip .black],
    types := [.creature], subtypes := [vampire, shaman],
    text := [flying, deathtouch, lifelink], power := stat 2, toughness := stat 3 }
