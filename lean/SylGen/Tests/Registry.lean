import Semantics.Definitions

open Lean
namespace SylGen.TestRegistry

def entries (env : Environment) : Array (String × String × Name) := Id.run do
  let mut entries := #[]
  for (_, rows) in Semantics.Registry.expectedTypeDispatchExt.getState env do
    for row in rows do
      entries := entries.push (row.bareName.toString, row.returnType.getString!, row.declaration)
  return entries

end SylGen.TestRegistry
