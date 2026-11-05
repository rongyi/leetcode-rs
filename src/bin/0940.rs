struct Solution;

impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut last = vec![0; 26];
        let mut total = 0;

        // Let's step through a concrete example with `s = "aba"` to see exactly why we multiply by 2 and why "keeping all existing subsequences" makes sense.

        // Suppose we are building subsequences character by character.

        // ---

        // ### Step 1: Processing `'a'` (Index 0)

        // Before seeing `'a'`, our set of non-empty subsequences is empty: `{}` (count = 0).

        // When we process the first `'a'`:

        // * **New subsequences formed by appending `'a'`:**
        // * Appending `'a'` to the empty sequence gives: `["a"]`

        // * **Existing subsequences carried over:**
        // * None: `{}`

        // * **Total subsequences so far:** `["a"]` (count = 1)

        // ---

        // ### Step 2: Processing `'b'` (Index 1)

        // Before `'b'`, our valid distinct subsequences are: **`["a"]`**.

        // When we process `'b'`, every existing subsequence can produce a new subsequence by attaching `'b'` to the end of it:

        // 1. **Keep all existing subsequences without attaching `'b'`:**
        // * `["a"]` (1 subsequence)

        // 2. **Create new subsequences by attaching `'b'` to existing ones:**
        // * Take `"a"` $\rightarrow$ append `'b'` $\rightarrow$ **`"ab"`** (1 subsequence)

        // 3. **Form the single-character subsequence `"b"` on its own:**
        // * **`"b"`** (1 subsequence)

        // Notice what happened to our count:

        // $$\text{New Total} = \underbrace{\text{Existing Count}}_{\text{keep existing ("a")}} + \underbrace{\text{Existing Count}}_{\text{append 'b' to existing ("ab")}} + \underbrace{1}_{\text{single 'b'}}$$

        // $$\text{New Total} = 1 + 1 + 1 = 3$$

        // Our set of distinct subsequences is now: **`["a", "ab", "b"]`**.

        // ---

        // ### Step 3: Processing `'a'` again (Index 2)

        // Before this second `'a'`, our set of valid subsequences is: **`["a", "ab", "b"]`** (count = 3).

        // If `'a'` were a completely new character, we would do the exact same doubling:

        // 1. **Keep all existing subsequences:** `["a", "ab", "b"]` (3)
        // 2. **Append `'a'` to all existing subsequences:** `["aa", "aba", "ba"]` (3)
        // 3. **Single character `"a"`:** `["a"]` (1)

        // Total would be $3 \times 2 + 1 = 7$.

        // **However, `'a'` is a duplicate!** Let's list what we generated:

        // * Existing kept: `"a"`, `"ab"`, `"b"`
        // * Appended `'a'`: `"aa"`, `"aba"`, `"ba"`
        // * Single `'a'`: `"a"`

        // Notice that **`"a"`** appears twice (once from existing, once from single `'a'`).

        // The duplicates generated when adding a duplicate character `c` are **exactly the subsequences that were formed the previous time `c` was added**.

        // ---

        // ### Summary of the Formula

        // At any step:

        // $$\text{new\_total} = \underbrace{\text{current\_dp}}_{\text{keep existing}} + \underbrace{\text{current\_dp}}_{\text{append } c \text{ to existing}} + \underbrace{1}_{\text{single character } c} - \underbrace{\text{last}[c]}_{\text{subtract duplicates}}$$

        // Combining the first two terms gives $2 \times \text{current\_dp}$:

        // $$\text{new\_total} = 2 \times \text{current\_dp} + 1 - \text{last}[c]$$
        for c in s.bytes() {
            let index = (c - b'a') as usize;
            let new_count = (total + 1) % MOD;
            total = ((total + new_count - last[index]) % MOD + MOD) % MOD;
            last[index] = new_count;
        }

        total
    }
}

fn main() {}
