template enchantedBecomesVanilla (colors : List Color) (subtype : Subtype) (power toughness : Nat) :
    StaticSpec := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := conjunction [ allAbilityLoss (enchanted creature),
      qualityChange it .sets
        (bundle { characteristics := { colors, types := [.creature], subtypes := [subtype] } }),
      getsBase it (amountLit power) (amountLit toughness) ]
}
