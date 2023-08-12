An L-System consists of an alphabet - that is a set of valid "words", an Axiom, that is an initial state of the system, i.e. a String or Vector of words and a set of transformation rules that will transform one string of words to another string of words. 

A generic definition of an alphabet is easily done by requiring a common type. 

So our Axiom would be simply a `Vec<T>` the only requirement for `T` being that we can determine equality between two `T` 

A transformation rule will simply be a Tuple of a `T` and the result of the transformation `Vec<T>` (since a transformation can result in more than one and also zero elements), and all of the rules will be a Vec of those Tuples.

It gets interesting when we take the output of such a system as input to a Turtle-Graphics style state-machine. 

It is tempting to try to have this work for `Box<dyn FnMut()>` directly, but this gets hairy quickly because determining Equality of functions is nontrivial. We could make a different implementation where instead of Eq we require Deref and compare the pointer-values instead of their targets.

To be able to incorporate context-aware L-Systems, we implement a version whose rules are set for `Vec<T>` instead of a simple `T` but we need some semantics for how rules are applied, i.e. rule precedence. 
I believe that the most straightforward semantics here are that for any particular slice of the axiom, at most one rule is applied per iteration, otherwise a single iteration could potentially infinitely recurse. 
This will also make the complex l-system behave like the simple one where all rules are for length 1. 

