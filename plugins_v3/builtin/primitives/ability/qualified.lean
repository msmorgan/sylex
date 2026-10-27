/-- `ability` with the restrictions a keyword or a granting effect adds to it [CR#113.10a]. -/
-- [CR#113.10a]
primitive qualified (ability : Ability) (timing : Option Timing := none)
    (limit : Option UsageLimit := none) (guard : Option Condition := none)
    (from_ : Option Zone := none) : Ability := {
  -- TODO(spelling): `ability` with the restrictions a keyword or a granting effect adds to it .
  body :=
    Semantics.Primitives.Ability.qualified ability timing limit guard from_
}
