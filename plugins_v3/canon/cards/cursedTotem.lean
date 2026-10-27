card cursedTotem : Spelled :=
  { name := "Cursed Totem", cost := [generic 2], types := [.artifact],
    text := [static (cantActivateAbilitiesOf isCreature)] }
