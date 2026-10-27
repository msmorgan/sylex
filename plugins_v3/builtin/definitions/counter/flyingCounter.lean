/-- [CR#122.1b] "A keyword counter on a permanent … causes that object to gain that keyword."
The counter is not an ability, so its conferral is the ability-free `property` flavor; but its
bearer gains the keyword in layer 6 [CR#613.1f], and that gained ability can be removed by
another effect [CR#113.12], applied in timestamp order [CR#613.7,613.7c]. -/
counter flyingCounter := {
  lexical := { lemma := "flying counter" },
  spelling := "flying counter",
  holder := .object,
  confers := [.property (Templates.abilityGrant .this (keyword (.named "flying")))]
}
