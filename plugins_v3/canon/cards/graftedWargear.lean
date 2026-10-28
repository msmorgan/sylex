/-- Grafted Wargear -/
card := {
  name := "Grafted Wargear",
  cost := [3],
  types := [artifact],
  subtypes := [equipment],
  text := [
    getsPt (equipped creature) (up 3) (up 2),
    whenever (does any (attachment unattached thisEquipment (a isPermanent)))
      (sacrifice (that permanent)),
    equip [0]
  ]
}
