/-- "<action>. <forbidden> can't happen", with no duration printed: later text that modifies the
meaning of `action` (`Duration.modifiesPrecedingAction`), a prohibition in force only for that
action's event, its subject a mention of its own read back from the action ("It can't be
regenerated"). A destroy with this rider is one no regeneration shield replaces
[CR#608.2c,701.19c]. -/
-- [CR#608.2c,701.19c]
template doUnderProhibition (action forbidden : Action) : Action := {
  spelling := "<action>. <forbidden> can't happen",
  body :=
    sequentially
      [action,
        establish (staticSpecCant any forbidden { })
          (some modifiesPrecedingAction)]
}
