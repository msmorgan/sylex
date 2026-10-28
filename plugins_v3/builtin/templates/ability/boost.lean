/-- What one +X/+Y counter confers [CR#122.1a]: "+X/+Y" to its bearer [CR#613.4c]. A counter
definition's conferral is per counter (`RegistryDefinition.counter`), so the deltas read no count. -/
-- [CR#122.1a]
-- [CR#613.4c]
template boost (power toughness : Delta Amount) : Conferral := {
  -- TODO(spelling): Context-dependent wording: +X/+Y; string substitution cannot express its omitted or inflected parts.
  body :=
    .property
      (characteristicChange this_
        [statEdit .power power,
          statEdit .toughness toughness])
}
