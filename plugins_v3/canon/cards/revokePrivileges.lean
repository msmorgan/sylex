/-- Revoke Privileges -/
card revokePrivileges : Spelled :=
  { name := "Revoke Privileges", cost := [generic 2, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction [ cant (enchanted creature) attack,
      cant (enchanted creature) block,
      cant (enchanted creature) (crew (allOf (hasSubtype
          vehicle))) ]) ] }
