# Sequents

Using [sequent calculus](https://en.wikipedia.org/wiki/Sequent_calculus), this tool attempts to prove a given formula in 1st order prepositional modal logic ('1OPML') as valid (ie. a tautology). Such a formula might look something like: `□(p∧q)→□p∧□q`.

1OPML (which adds the modal operators {◇, □}) also subsumes 1st order _prepositional logic_. So, formulae like this `p→(p∨q→p)∧¬p` work just fine.

## User Input

The takes as input a single formula in 1OPML. This input can be in unicode, ascii or an arbitrary mix[1]. Multiple input arguments will be taken to be as part of the same formula (whitespace is ignored). However, the special input word `--unicode` is detected and escaped. This switches the program from the default ascii mode to unicode[1].

[1] See the section `Unicode or ASCII`

## Unicode or ASCII
Internally, the implementation represents the formulae using the unicode logic symbols {→,⇒,¬,∧,∨,◇,□}. For ease-of-use and for compatibility for consoles that may not display these characters correctly, ascii mode is enabled by default. This means that all _outputs_ are given by their ascii representations, as shown in the following table.

| Operator     | Unicode | ASCII | Also accepted ASCII |
| :------------- | :------------- |
|      | ⇒ | =>    |    |
| not   | ¬ | -     | ~  |
| and   | ∧ | &     | /\ |
| or    | ∨ | V     | \/ |
| implication   |→ | ->     |   |
| box | □ | [] | |
| diamond | ◇ | <> ||

## Rules

| # | Rule Left | Rule Right
| :------ | :------- |
|1| A, `¬φ` ⇒ B | A ⇒ `φ`, B
|2| A ⇒ `¬φ`, B | A, `φ` ⇒ B
|3| A, `φ∧ψ` ⇒ B | A, `φ, ψ` ⇒ B
|4| A ⇒ φ ∨ ψ, B | A ⇒ φ, ψ, B
|5| A ⇒ `φ∧ψ`, B | A ⇒ `φ`, B and A ⇒ `ψ`, B
|6| A, `φ∨ψ` ⇒ B | A, `φ` ⇒ B and A, `ψ` ⇒ B
|7| p1,...,pn,`◇φ1,...,◇φm` ⇒ q1,...,qk,◇ψ1,...,◇ψj | `φi` ⇒ ψ1,...,ψj for some i ∈ [1,m]

## Output

An example of an execution with the input formula `□(p∧q)→□p∧□q`, given in ascii as `[](p&q)->[]p&[]q`, using the optional `--unicode` flag to enable unicode-formatted output.

After a preprocessing step to get rid of operators {→,□} a single `Proof` instance is generated. Presence of some operators sometimes necessitate other, smaller `Proof` instances to determine validity. These 'inner' proofs are indented to indicate their relationship with the outer proof.

```
λ .\sequents.exe --unicode "[](p&q)->[]p&[]q"
Given: □(p∧q)→□p∧□q
...preprocessed to: ¬¬◇¬(p∧q)∨¬◇¬p∧¬◇¬q
starting with:   ⇒  ¬¬◇¬(p∧q)∨¬◇¬p∧¬◇¬q...
* Prove :   ⇒  ¬¬◇¬(p∧q)∨¬◇¬p∧¬◇¬q
  rule 4:   ⇒  ¬¬◇¬(p∧q),¬◇¬p∧¬◇¬q
  rule 2: ¬◇¬(p∧q)  ⇒  ¬◇¬p∧¬◇¬q
  rule 1:   ⇒  ¬◇¬p∧¬◇¬q,◇¬(p∧q)
  rule 6: valid if both... (valid)
    * Prove :   ⇒  ◇¬(p∧q),¬◇¬p
      rule 2: ◇¬p  ⇒  ◇¬(p∧q)
      rule 7: valid if any... (valid)
        * Prove : ¬p  ⇒  ¬(p∧q)
          rule 1:   ⇒  ¬(p∧q),p
          rule 2: p∧q  ⇒  p
          rule 3: p,q  ⇒  p
          valid!
    * Prove :   ⇒  ◇¬(p∧q),¬◇¬q
      rule 2: ◇¬q  ⇒  ◇¬(p∧q)
      rule 7: valid if any... (valid)
        * Prove : ¬q  ⇒  ¬(p∧q)
          rule 1:   ⇒  ¬(p∧q),q
          rule 2: p∧q  ⇒  q
          rule 3: p,q  ⇒  q
          valid!
VALID
```
