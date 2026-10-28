card := {
  name := "Glacial Chasm",
  types := [land],
  text := [
    cumulativeUpkeep (payLife 2),
    when (does thisLand enter) (sacrifice (a isLand)),
    cant (allOf [isCreature, youControl]) attack,
    replace (would any (dealDamage anyAmount you)) (instead (preventDamage allDamage))
  ]
}
