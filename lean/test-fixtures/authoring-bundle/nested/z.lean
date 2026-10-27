action bundleDraw := { body := draw 1 }
primitive bundleAtom : Predicate := { body := .isCard }
action bundleShared := { body := bundleDraw }
