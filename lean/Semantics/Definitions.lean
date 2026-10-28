import Semantics.Primitives

/-!
# Semantics.Definitions

The builtin vocabulary and rules, loaded from individual declarations in `plugins_v3/builtin`.
Checker-level card-type values live in `Semantics.Words`.
-/

namespace Semantics.Definitions

open Semantics.Templates

load_bundle "../plugins_v3/builtin" templatesIn Semantics.Templates

end Semantics.Definitions

namespace Semantics.Templates
export Semantics.Templates.NounPhrase (target)
end Semantics.Templates
