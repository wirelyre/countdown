partall ← { # all nontrivial partitions
  Step ← {𝕩⊸∾¨↕2+⌈´¯1∾𝕩}
  all ← (1⊸↓)¨ (∾Step¨)⍟⊣⟜⟨⟨⟩⟩¨ ↕7
  {(⊔⟜𝕩)¨(≠𝕩)⊑all}
}
Part2 ← { # all ordered two-partitions with nonempty first
  ⟨⟩:⟨⟩; ⟨a⟩:<⟨a⟩‿"";
  (∾⟜⟨""⟩)⌾⊑(⊔⟜𝕩)¨¯1↓⥊∾⌜´/⟜⟨0‿1⟩≠𝕩
}

I ← {𝕨𝕊"":"";(⊑𝕩)∾∾∾⟜(⊑𝕨)¨1⊸↓𝕩} # intersperse positive
J ← {𝕨𝕊"":"";𝕨(⊑⊸I∾1⊸⊑˜)𝕩}      # intersperse negative
  ! ("+-" I "abc") ≡ "ab+c+"
  ! ("+-" J "abc") ≡ "ab+c+-"
K ← I⟜⊑ ∾ {𝕨 J 1⊑𝕩∾⟨⟨⟩⟩} # resolve both
  ! ("+-" K ⟨⟨"ab*","c"⟩,⟨"d", "ef/"⟩⟩) ≡ "ab*c+def/+-"


# Cartesian product of list of tuples
Prods ← {⟨⟩:⟨⟩; ⟨a⟩:a; ⥊ <⊸∾⌜´ (<¨⌾(¯1⊸⊑)) 𝕩}

All ← {
  𝕨𝕊⟨⟩:  !"cannot arrange nothing";
  𝕨𝕊⟨a⟩: ⟨⟨a⟩⟩;
  r ← (⌽𝕨)⊸All¨¨ Partall 𝕩
  ∾ (0⊑𝕨)⊸K¨∘Part2¨ ∾Prods¨ r
}

# •Out¨ ⟨"a","b","c","d","e","f"⟩ # ones
# •Out¨ ∾⟨"+-","*/"⟩⊸All¨ ({(1<+´¨)⊸/ ⥊∾⌜´/⟜⟨0‿1⟩≠𝕩}/¨<) "abcdef"
# •Out¨ ∾⟨"*/","+-"⟩⊸All¨ ({(1<+´¨)⊸/ ⥊∾⌜´/⟜⟨0‿1⟩≠𝕩}/¨<) "abcdef"



ToInst ← {
  Step ← {in 𝕊 ⟨stack, inst, top⟩: {
    ⊑in∊"abcdef"? ⟨(1+in-'a')∾stack, inst, top⟩;
    ⟨rhs,lhs⟩ ← 2↑stack
    ⟨top∾2↓stack, inst∾⟨⟨lhs,rhs,in⟩⟩, top+1⟩
  }}
  ⟨⟨res⟩, inst, top⟩ ← Step´ ⌽⟨⟨⟨⟩,⟨⟩,7⟩⟩ ∾ 𝕩
  nop ← ⟨0, res, '+'⟩
  inst ∾ (5-≠inst)/⟨nop⟩
}

ToC ← { ⟨lhs, rhs, op⟩:
  op ↩ ⟨"Add","Sub","Mul","Div"⟩⊑˜⊑"+-*/"⊐op
  "{ .lhs="∾(•Fmt lhs)∾", .rhs="∾(•Fmt rhs)∾", .op="∾op∾"},
"
}



ones ← ⟨"a","b","c","d","e","f"⟩
adds ← ∾⟨"+-","*/"⟩⊸All¨ ({(1<+´¨)⊸/ ⥊∾⌜´/⟜⟨0‿1⟩≠𝕩}/¨<) "abcdef"
muls ← ∾⟨"*/","+-"⟩⊸All¨ ({(1<+´¨)⊸/ ⥊∾⌜´/⟜⟨0‿1⟩≠𝕩}/¨<) "abcdef"
count ← (≠ones) + (≠adds) + (≠muls)

•Out "
#include ""compute.h""

int computations_count = "∾(•Fmt count)∾";
struct Instruction computations[] = {
"
  {•Out∾´ToC¨ToInst𝕩}¨ ones
  {•Out∾´ToC¨ToInst𝕩}¨ adds
  {•Out∾´ToC¨ToInst𝕩}¨ muls
•Out "};"