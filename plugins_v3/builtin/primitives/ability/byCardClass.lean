/-- `onPermanent` on a permanent card and `onInstantOrSorcery` on an instant or sorcery card: a
keyword the rules define by Card Class [CR#110.4,110.4a]. -/
-- [CR#110.4,110.4a]
primitive byCardClass (onPermanent onInstantOrSorcery : Ability) : Ability := {
  -- TODO(spelling): `onPermanent` on a permanent card and `onInstantOrSorcery` on an instant or sorcery card: a keyword the rules define by Card Class .
  body :=
    Semantics.Primitives.Ability.byCardClass onPermanent onInstantOrSorcery
}
