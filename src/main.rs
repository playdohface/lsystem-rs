use rand::Rng;

/// A simple, generic implementation of an L-System. (https://en.wikipedia.org/wiki/L-system)
/// The alphabet will be all instances of T that actually occur in either the axiom or the rules
/// There can be multiple rules attached to the same element, they are applied in order
/// Any element without a rule is treated as a constant
fn lsystem<T: PartialEq + Clone>(
    axiom: Vec<T>,
    rules: &Vec<(T, Vec<T>)>,
    iterations: u32,
) -> Vec<T> {
    if iterations == 0 {
        return axiom;
    }
    let mut result = Vec::with_capacity(axiom.len() * 2);
    'elements: for elem in axiom {
        for (original, replacement) in rules {
            if elem == *original {
                result.append(&mut replacement.clone());
                continue 'elements;
            }
        }
        result.push(elem);
    }
    if iterations > 1 {
        lsystem(result, rules, iterations - 1)
    } else {
        result
    }
}

/// a refined implementation of a deterministic L-System, that takes as its rules pairs of Vec<T>, the first one will be replaced by the second
/// any slice of the axiom will have at most one rule applied to it per iteration, and the first one matching wins
/// this can be used to implement context-aware L-systems
fn complex_lsystem<T: PartialEq + Clone>(
    axiom: Vec<T>,
    rules: &Vec<(Vec<T>, Vec<T>)>,
    iterations: u32,
) -> Vec<T> {
    if iterations == 0 {
        return axiom;
    }
    let mut result: Vec<T> = Vec::with_capacity(axiom.len() * 2);
    let mut i = 0;
    'outer: while i < axiom.len() {
        'inner: for (original, replacement) in rules {
            if i + original.len() > axiom.len() {
                continue 'inner;
            }
            if axiom[i..i + original.len()] == *original.as_slice() {
                result.append(&mut replacement.clone());
                i += original.len();
                continue 'outer;
            }
        }
        result.push(axiom[i].clone());
        i += 1;
    }
    if iterations > 1 {
        complex_lsystem(result, rules, iterations - 1)
    } else {
        result
    }
}

/// an implementation of a non-deterministic L-system
/// Each rule is a tuple of (original, replacement, chance)
/// where original is a Vec<T> that will be replaced by replacement with a chance between 0.0 and 1.0
/// Note that each chance is calculated individually - so to express that A will be replaced either by B or C with a
/// 50% chance each, the rules are `vec![(vec![A], vec![B], 0.5), (vec![A], vec![C], 1.0)]`
fn random_lsystem<T: PartialEq + Clone>(
    axiom: Vec<T>,
    rules: &Vec<(Vec<T>, Vec<T>, f32)>,
    iterations: u32,
) -> Vec<T> {
    if iterations == 0 {
        return axiom;
    }
    let mut result: Vec<T> = Vec::with_capacity(axiom.len() * 2);
    let mut rng = rand::thread_rng();
    let mut i = 0;
    'outer: while i < axiom.len() {
        'inner: for (original, replacement, chance) in rules {
            if i + original.len() > axiom.len() {
                continue 'inner;
            }
            if axiom[i..i + original.len()] == *original.as_slice()
                && rng.gen_range(0.0..=1.0) <= *chance
            {
                result.append(&mut replacement.clone());
                i += original.len();
                continue 'outer;
            }
        }
        result.push(axiom[i].clone());
        i += 1;
    }
    if iterations > 1 {
        random_lsystem(result, rules, iterations - 1)
    } else {
        result
    }
}

/// A fully generic implementation of an L-System
/// axiom: starting state of the system
/// rules: tuple of a pattern of what is to be transformed and a function of the transformation to be applied
fn arbitrary_lsystem<T: PartialEq + Clone>(
    axiom: Vec<T>,
    rules: &Vec<(Vec<T>, impl Fn(Vec<T>) -> Vec<T>)>,
    iterations: u32,
) -> Vec<T> {
    if iterations == 0 {
        return axiom;
    }
    let mut result: Vec<T> = Vec::with_capacity(axiom.len() * 2);
    let mut i = 0;
    'outer: while i < axiom.len() {
        'inner: for (original, transform) in rules {
            if i + original.len() > axiom.len() {
                continue 'inner;
            }
            if axiom[i..i + original.len()] == *original.as_slice() {
                result.append(&mut transform(original.clone()));
                i += original.len();
                continue 'outer;
            }
        }
        result.push(axiom[i].clone());
        i += 1;
    }
    if iterations > 1 {
        arbitrary_lsystem(result, rules, iterations - 1)
    } else {
        result
    }
}

fn main() {
    let a = 'A';
    let b = 'B';

    let axiom = vec![a];
    let rules = vec![(a, vec![a, b]), (b, vec![a])];
    let complexrules = vec![(vec![a], vec![a, b]), (vec![b], vec![a])];
    let randomrules = vec![(vec![a], vec![a, b], 0.5), (vec![b], vec![a], 0.75)];
    fn transform<T: Eq + Clone>(orig: Vec<T>) -> Vec<T> {
        let mut rng = rand::thread_rng();
        let times: u16 = rng.gen_range(0..=3);
        let mut res: Vec<T> = Vec::new();
        for _ in 0..times {
            res.append(&mut orig.clone());
        }
        res
    }

    let arbitraryrules = vec![(vec![a], transform)];
    for n in 0..7 {
        println!("{:?}", lsystem(axiom.clone(), &rules, n));
        println!("{:?}", complex_lsystem(axiom.clone(), &complexrules, n));
        println!("{:?}", random_lsystem(axiom.clone(), &randomrules, n));
        println!("{:?}", arbitrary_lsystem(axiom.clone(), &arbitraryrules, n));
    }
}
