/// Problem:
/// Given an array of integers, do an inplace permutation such that
/// we end up with all zeros in the beginning of the array (another variant
/// is at the end)
///
pub fn push_zero_start(v: &mut Vec<i32>) -> usize {
    // loop invariant:
    //
    // idea sketch: [0, 0, 0, 1, 2, 5, 0, 10 ...]
    //                        ^     ^
    //                        i     j
    // (1) v[n] = 0  | n < i <= j <= v.len()
    // (2) v[n] != 0 | i <= n < j
    //

    // Initialization
    // ==============
    // i = 0, j = 0
    // (1) and (2) hold as they represent the empty set
    //
    // Note that they hold also in the case of a zero filled array.
    // at the end of the loop (2) will be an empty set as i = j = v.len()
    // and there's a strict inequality i < j.
    // Indeed, i == j <=> it's a zero filled array (TODO prove)
    let mut i: usize = 0;
    let mut j: usize = 0;

    // Preservation:
    // =============
    // we start with invariant
    //  (1) v[n] = 0 for n < i.
    //  (2) v[n] != 0 for i <= n < j
    //
    // case analysis:
    //
    //  case:
    //      v[j] = 0:
    //          (1)
    //          v[i] <- 0 (from the swap)
    //          i <- i + 1
    //          => v[n] = 0 | n < i
    //          => invariant preserved
    //          (2)
    //          v[n] != 0 for i <= n < j
    //              if i == j then this set is empty we increment both i and j
    //              so we preserve an empty set.
    //              if i < j, then given that v[n] = 0 for i < n
    //                  we swap a non 0 in v[j] (v[i] != 0 from precondition)
    //                  so v[n] != for n in [i+1..j]
    //                  => invariant preserved
    //
    //  else:
    //     (1) i doesn't change, invariant preserved.
    //     (2) j <- j + 1, and v[j] != 0 so invariant preserved.
    //
    while j < v.len() {
        if v[j] == 0 {
            v.swap(i, j);
            i += 1;
        }
        j += 1;
    }
    //
    // Postcondition:
    // ==============
    // (1) holds by maintenance
    // (2) !(j < v.len()) and invariant j <= v.len() => j = v.len()
    // So we have an i s.t. v(n) = 0 for n < i and v[n] != o fir i <= n < v.len()
    //
    // Termination:
    // ============
    // The decrementing function can be DF = (v.len() - j), given that j >= v.len()
    // then DF >= 0 and j is increasing in every case of the loop then we have a termination.
    //
    // There's still need to prove???:
    // - that the order is preserved
    // - that we don't have a case where we're for instance filling
    // the array with one value ..
    i
}

#[cfg(test)]
mod tests {
    use quickcheck_macros;

    use crate::{arrays::push_zero_start, counter::Counter};

    #[quickcheck_macros::quickcheck]
    fn prop_zero_prefix(xs: Vec<i32>) -> bool {
        let mut xs = xs.clone();
        let i = push_zero_start(&mut xs);
        i <= xs.len() && (i == xs.len() && xs.iter().all(|e| *e == 0))
            || (0..xs.len()).all(|n| (xs[n] == 0 && n < i) || (xs[n] != 0 && n >= i))
    }

    #[quickcheck_macros::quickcheck]
    fn prop_keep_elements(xs: Vec<i32>) -> bool {
        let counter = Counter::<i32>::from(xs.clone().into_iter());
        let mut xs = xs.clone();

        let _ = push_zero_start(&mut xs);
        let after_counter = Counter::<i32>::from(xs.clone().into_iter());

        counter == after_counter
    }
}
