/-- Delivery Moogle -/
card deliveryMoogle : Spelled :=
  { name := "Delivery Moogle", cost := [generic 3, pip .white], types := [.creature],
    subtypes := [moogle],
    text :=
      [ flying,
        when (does thisCreature enter)
          (sequentially
            [ searchLibraryOrGraveyard (a (and [isArtifact, hasManaValue .atMost 2])),
              reveal it,
              putInto it hand,
              doIf (did you (search yourLibrary) thisWay) shuffle ]) ],
    power := stat 3, toughness := stat 2 }
