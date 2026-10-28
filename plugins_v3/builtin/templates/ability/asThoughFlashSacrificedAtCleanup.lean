/-- "You may cast this spell as though it had flash. If you cast it any time a sorcery couldn't
have been cast, the controller of the permanent it becomes sacrifices it at the beginning of the
next cleanup step." (Armor of Thorns, Mystic Veil, Ward of Lights …): one paragraph, so one
ability [CR#113.2c], the permission with the second sentence its rider (`DeonticMods.rider`).
"If you cast it" is a cast made under that permission, its cast-timing check [CR#307.5a] reads
that cast, and the delayed trigger is created as the permanent it becomes is put onto the
battlefield [CR#608.3g]. A cast under another effect's permission is not that cast, so nothing
is sacrificed (Mystic Veil ruling, 2022-12-08). -/
template asThoughFlashSacrificedAtCleanup : Ability := {
  spelling := "You may cast this spell as though it had flash. If you cast it any time a sorcery couldn’t have been cast, the controller of the permanent it becomes sacrifices it at the beginning of the next cleanup step.",
  body :=
    static (can you (cast thisSpell) (asThough := asThoughHadFlash it)
      (rider := some (doIf (did you (cast it) (within thisGame (some (not asSorcery))))
        (atNext cleanup (instruct (controllerOf (becomes it)) (sacrifice it))))))
}
