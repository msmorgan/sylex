card := {
  name := "Prismari, the Inspiration",
  cost := [5, blue, red],
  supertypes := [legendary],
  types := [creature],
  subtypes := [elder, dragon],
  text := [
    flying,
    ward (payLife 5),
    abilityGrant (allOf [isInstantOrSorcery, isSpell, did you (cast which) thisObject]) storm
  ],
  power := 7,
  toughness := 7
}
