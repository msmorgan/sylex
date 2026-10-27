/-- [CR#120.3e] "Damage dealt to a creature by a source with neither wither nor infect causes
that much damage to be marked on that creature." Wither and infect replace the marking
[CR#120.3d]. -/
template creatureDamageMarking : Conferral := {
  -- TODO(spelling): Damage dealt to a creature by a source with neither wither nor infect causes
  -- that much damage to be marked on that creature.; this is a rules conferral, whose inherent or
  -- ability-free origin cannot be encoded by card-text substitution.
  body :=
    damageResult (markDamage this_ thatMuch)
}
