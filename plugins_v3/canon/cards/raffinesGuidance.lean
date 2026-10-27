/-- Raffine's Guidance -/
card raffinesGuidance : Spelled :=
  { name := "Raffine's Guidance", cost := [pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (getsPt (enchanted creature) (up 1) (up 1)),
        static (can you
                  (cast this_ (from_ := yourGraveyard)
                     (payment := payingInstead [generic 2, pip .white]))) ] }
