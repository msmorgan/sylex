/-- [CR#310.12a] "As a Siege enters the battlefield, its controller must choose its protector
from among their opponents. Only an opponent of a Siege's controller can be its protector." A
rule and no ability of the Siege [CR#113.12], made as part of its entering [CR#614.1c,614.12]. -/
template siegeProtector : Conferral := {
  -- TODO(spelling): As a Siege enters the battlefield, its controller must choose its protector
  -- from among their opponents. Only an opponent of a Siege's controller can be its protector.;
  -- this is a rules conferral, whose inherent or ability-free origin cannot be encoded by card-text
  -- substitution.
  body :=
    .property (entersChoosingPlayer thisSiege anOpponent)
}
