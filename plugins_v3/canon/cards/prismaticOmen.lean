card prismaticOmen : Spelled :=
  { name := "Prismatic Omen", cost := [generic 1, pip .green], types := [.enchantment],
    text :=
      [ static (qualityChange (allOf (and [isLand, youControl])) .adds
          (everyTypeOf .basicLand)) ] }
