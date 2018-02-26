# Sequents

This is a [sequent calculus](https://en.wikipedia.org/wiki/Sequent_calculus) validity checker ('validator') for 1st order prepositional modal logic ('1OPML'), written in [Rust](https://www.rust-lang.org), a systems programming language.

The validator attempts to prove a given formula in 1OPML is _valid_ (ie. a tautology). Such a formula might look something like: `□(p∧q)→□p∧□q`.

1OPML (which adds the modal operators {◇, □}) also subsumes 1st order _prepositional logic_. So, formulae like this `p→(p∨q→p)∧¬p` work just fine.

## Setup

The validator must first be compiled from source on your machine to guarantee that it runs. First, you might consider trying your luck with one of the pre-compiled binaries provided in the `.../precompiled_binaries` directory. Failing that, follow these steps to compile it yourself:
1. If you don't have `rustc` and `cargo` installed on your machine, acquire them by using [rustup](https://www.rustup.rs), the Rust toolchain manager. Follow the default prompts to install the default 'stable' release of Rust and Cargo.
1. Clone this git repository to your local machine and navigate to it.
1. Use `cargo build --release` to compile the binary with optimizations enabled into `.../target/release`. It will be named `sequents` (or `sequents.exe` on Windows).

## User Input

The validator takes as input a single formula in 1OPML. This input can be in unicode, ascii or an arbitrary mix[1]. Beware of your shell parsing some of the ascii characters in unintended ways (Perhaps surround your formula with `""` as in `sequents "<>p->-[]pVq"`). Multiple input arguments will be taken to be as part of the same formula (whitespace is ignored). However, the special input word `--unicode` is detected and escaped. This switches the program from the default ascii mode to unicode[1].


[1] See the section `Unicode or ASCII`

## Unicode or ASCII
Internally, the validator's implementation represents logical formulae using unicode characters {→,⇒,¬,∧,∨,◇,□}. For ease-of-use and for compatibility for consoles that may not display these characters correctly, ascii mode is enabled by default. This means that all _outputs_ are given by their ascii representations, as shown in the following table.

| Operator     | Unicode | ASCII | Also accepted input |
| :--------- | :------- |:------- |:------- |
|      | ⇒ | =>    |    |
| implication   |→ | ->     |   |
| box | □ | [] | |
| top | T | T | ⊤ |
| bottom | F | F | ⊥ |
| diamond | ◇ | <> | |
| variable | a-z | a-z | |
| not   | ¬ | -     | ~  |
| and   | ∧ | &     | /\ |
| or    | ∨ | V     | \/ |

## Rules

To determine validity, the validator relies on the following rewrite rules, which allow a sequent to be transformed and simplified from the form on the left to the form on the right. These names are shown in the output to make the process easier to follow. 

Name | Rule Left | Rule Right
 :------ | :------- | :-------
lneg| A, `¬φ` ⇒ B | A ⇒ `φ`, B
rneg| A ⇒ `¬φ`, B | A, `φ` ⇒ B
land| A, `φ∧ψ` ⇒ B | A, `φ, ψ` ⇒ B
r_or| A ⇒ φ ∨ ψ, B | A ⇒ φ, ψ, B
rand| A ⇒ `φ∧ψ`, B | A ⇒ `φ`, B and A ⇒ `ψ`, B
l_or| A, `φ∨ψ` ⇒ B | A, `φ` ⇒ B and A, `ψ` ⇒ B
diam| A,`◇φ1,...,◇φm` ⇒ B,◇ψ1,...,◇ψj | `φi` ⇒ B for some i ∈ [1,m]
ltop| A, `⊤` ⇒ B | A ⇒ B
rbot| A ⇒ `⊥`, B | A ⇒ B

## Output

After a preprocessing step to get rid of operators {→,□} a single `Proof` instance is generated. Presence of some operators sometimes necessitate other, smaller `Proof` instances to determine validity. These 'inner' proofs are indented to indicate their relationship with the outer proof.

Below is an example of an execution with the input formula `□(p∧q)→□p∧□q`, given in ascii as `[](p&q)->[]p&[]q`, using the optional `--unicode` flag to enable unicode-formatted output.

```
Given: □(p∧q)→□p∧□p
...preprocessed to: ¬¬◇¬(p∧q)∨(¬◇¬p∧¬◇¬p)
starting with:   ⇒  ¬¬◇¬(p∧q)∨(¬◇¬p∧¬◇¬p)...
* Prove:   ⇒  ¬¬◇¬(p∧q)∨(¬◇¬p∧¬◇¬p)
  [r_or]   ⇒  ¬¬◇¬(p∧q),¬◇¬p∧¬◇¬p
  [rneg] ¬◇¬(p∧q)  ⇒  ¬◇¬p∧¬◇¬p
  [lneg]   ⇒  ¬◇¬p∧¬◇¬p,◇¬(p∧q)
  [rand] valid if both... (valid)
    * Prove:   ⇒  ◇¬(p∧q),¬◇¬p
      [rneg] ◇¬p  ⇒  ◇¬(p∧q)
      [diam] valid if any... (valid)
        * Prove: ¬p  ⇒  ¬(p∧q)
          [lneg]   ⇒  ¬(p∧q),p
          [rneg] p∧q  ⇒  p
          [land] p,q  ⇒  p
          valid!
    * Prove:   ⇒  ◇¬(p∧q),¬◇¬p
      [rneg] ◇¬p  ⇒  ◇¬(p∧q)
      [diam] valid if any... (valid)
        * Prove: ¬p  ⇒  ¬(p∧q)
          [lneg]   ⇒  ¬(p∧q),p
          [rneg] p∧q  ⇒  p
          [land] p,q  ⇒  p
          valid!
VALID!
```

## Counter-models

In the event the input formula is invalid, a counter-model is also output. Below is an example of an execution output including such a counter-model. In these outputs, world `1` is always the world that invalidates the given formula.

```
Given: ◇p→◇◇⊤
...preprocessed to: ¬◇p∨◇◇⊤
starting with:   ⇒  ¬◇p∨◇◇⊤...
* Prove:   ⇒  ¬◇p∨◇◇⊤
  [r_or]   ⇒  ¬◇p,◇◇⊤
  [rneg] ◇p  ⇒  ◇◇⊤
  [diam] valid if any... (invalid)
    * Prove: p  ⇒  ◇⊤
      invalid!
INVALID!
Counter-example:
Model:
  worlds: {1, 2}
  access fn: {(1, 2)}
  value fn: {
    p: {2}
  }
```
