card bundleCard := {
  name := "Bundle fixture",
  types := [sorcery],
  text := [spell bundleDraw],
}
template bundleQuality : Predicate := { body := bundleAtom }
keyword bundleShared := { body := [spell bundleShared] }
