/-- [CR#310.9a] "As a battle enters the battlefield, its controller chooses a player to be its
protector. Which players may be chosen as its protector are determined by its battle type …
If it has no battle types, only its controller can be its protector." Siege is the one battle
type [CR#205.3q], and the Siege row makes its own choice among its controller's opponents
[CR#310.12a], so this row is the choice of a battle that is no Siege, and the only player it may
choose is its controller: a choice is of an indefinite, so it chooses "a player not other than
you". A rule and no ability of the battle [CR#113.12], so a `property`; the choice is made as
part of the battle's entering [CR#614.1c,614.12]. -/
template battleProtector : Conferral := {

  -- TODO(spelling): As a battle enters the battlefield, its controller chooses a player to be its
  -- protector. Which players may be chosen as its protector are determined by its battle type … If
  -- it has no battle types, only its controller can be its protector.; this is a rules conferral,
  -- whose inherent or ability-free origin cannot be encoded by card-text substitution.
  body :=
    Semantics.Conferral.property
      (Semantics.Templates.StaticSpec.onlyUnless
        (Semantics.Templates.StaticSpec.entersChoosingPlayer Semantics.Templates.NounPhrase.this_
          (Semantics.Templates.NounPhrase.a
            (Semantics.Templates.Predicate.and
              [Semantics.Templates.Predicate.isPlayer,
                Semantics.Templates.Predicate.not (Semantics.Templates.Predicate.otherThan Semantics.Templates.NounPhrase.you)])))
        (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.this_
          (Semantics.Templates.Predicate.hasSubtype Semantics.Definitions.siege)))
}
