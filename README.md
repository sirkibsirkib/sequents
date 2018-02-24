# Sequents

This is a [sequent calculus](https://en.wikipedia.org/wiki/Sequent_calculus) validity checker ('validator') for 1st order prepositional modal logic ('1OPML'), written in [Rust](https://www.rust-lang.org), a systems programming language.

The validator attempts to prove a given formula in 1OPML is _valid_ (ie. a tautology). Such a formula might look something like: `□(p∧q)→□p∧□q`.

1OPML (which adds the modal operators {◇, □}) also subsumes 1st order _prepositional logic_. So, formulae like this `p→(p∨q→p)∧¬p` work just fine.

## Setup

As the validator is written in Rust, the binary must first be compiled on your machine. Follow these steps:
1. If you don't have rust already, acquire it by using [rustup](https://www.rustup.rs), the Rust toolchain manager. Follow the default online prompts until both the Rust compiler and the Rust package manager, _cargo_, are installed. The standard 'stable' release version of Rust are sufficient.
1. Clone this git repository to your local machine and navigate to it.
1. Use `cargo build --release` to compile the binary with optimizations enabled.
1. The binary will be found in `/target/release/` and will be called something like `sequents` or `sequents.exe`.

## User Input

The validator takes as input a single formula in 1OPML. This input can be in unicode, ascii or an arbitrary mix[1]. Multiple input arguments will be taken to be as part of the same formula (whitespace is ignored). However, the special input word `--unicode` is detected and escaped. This switches the program from the default ascii mode to unicode[1].

[1] See the section `Unicode or ASCII`

## Unicode or ASCII
Internally, the validator's implementation represents logical formulae using unicode characters {→,⇒,¬,∧,∨,◇,□}. For ease-of-use and for compatibility for consoles that may not display these characters correctly, ascii mode is enabled by default. This means that all _outputs_ are given by their ascii representations, as shown in the following table.

| Operator     | Unicode | ASCII | Also accepted ASCII |
| :------------- | :------------- |
|      | ⇒ | =>    |    |
| implication   |→ | ->     |   |
| box | □ | [] | |
| top | ⊤ | T | |
| bottom | ⊥ | F | |
| diamond | ◇ | <> | |
| not   | ¬ | -     | ~  |
| and   | ∧ | &     | /\ |
| or    | ∨ | V     | \/ |

## Rules

To determine validity, the validator relies on the following rewrite rules. These indices are used in the output to make the process easier to follow. Knowing the rules is not at all necessary to use the solver.

| Name | Rule Left | Rule Right
| :------ | :------- |
|lneg| A, `¬φ` ⇒ B | A ⇒ `φ`, B
|rneg| A ⇒ `¬φ`, B | A, `φ` ⇒ B
|land| A, `φ∧ψ` ⇒ B | A, `φ, ψ` ⇒ B
|r_or| A ⇒ φ ∨ ψ, B | A ⇒ φ, ψ, B
|rand| A ⇒ `φ∧ψ`, B | A ⇒ `φ`, B and A ⇒ `ψ`, B
|l_or| A, `φ∨ψ` ⇒ B | A, `φ` ⇒ B and A, `ψ` ⇒ B
|diam| A,`◇φ1,...,◇φm` ⇒ B,◇ψ1,...,◇ψj | `φi` ⇒ B for some i ∈ [1,m]
|ltop| A, `⊤` ⇒ B | A ⇒ B
|rbot| A ⇒ `⊥`, B | A ⇒ B

## Output

After a preprocessing step to get rid of operators {→,□} a single `Proof` instance is generated. Presence of some operators sometimes necessitate other, smaller `Proof` instances to determine validity. These 'inner' proofs are indented to indicate their relationship with the outer proof.

Below is an example of an execution with the input formula `□(p∧q)→□p∧□q`, given in ascii as `[](p&q)->[]p&[]q`, using the optional `--unicode` flag to enable unicode-formatted output.

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
