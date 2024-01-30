///
///Reverse a stack(vec) with the help of a temporary stack(vec)
///
use minivec::MiniVec;

fn lesser_or_greater<T: PartialOrd>(peeked: &T, t: &T, ascending: bool) -> bool {
    if ascending {
        *peeked < *t
    } else {
        *peeked > *t
    }
}

pub fn sort_with_aux_stack<T: PartialOrd>(mut stack: MiniVec<T>, ascending: bool) -> MiniVec<T> {
    if stack.len() < 2 {
        return stack;
    }

    let mut temp_v = MiniVec::with_capacity(stack.len());

    while let Some(t) = stack.pop() {
        while temp_v
            .peek()
            .is_some_and(|peeked| lesser_or_greater(peeked, &t, ascending))
        {
            stack.push(temp_v.pop().unwrap());
        }
        temp_v.push(t);
    }
    while let Some(t) = temp_v.pop() {
        stack.push(t);
    }
    stack
}
///Not advisable for large sizes because it uses recursion
pub fn sort<T: PartialOrd>(mut stack: MiniVec<T>, ascending: bool) -> MiniVec<T> {
    if stack.len() < 2 {
        return stack;
    }
    sort_rec(&mut stack, ascending);
    stack
}

fn sort_rec<T: PartialOrd>(stack: &mut MiniVec<T>, ascending: bool) {
    if stack.len() < 2 {
        return;
    }

    let elem = stack.remove(stack.len() - 1);
    sort_rec(stack, ascending);
    insert(stack, elem, ascending);
}

fn insert<T: PartialOrd>(stack: &mut MiniVec<T>, elem: T, ascending: bool) {
    if stack.is_empty()
        || stack
            .peek()
            .is_some_and(|peeked| lesser_or_greater(peeked, &elem, ascending))
    {
        stack.push(elem);
        return;
    }
    let popped = stack.remove(stack.len() - 1);
    sort_rec(stack, ascending);
    insert(stack, elem, ascending);
    stack.push(popped);
}

#[cfg(test)]
mod tests {
    use super::*;
    use minivec::mv;
    use rand::Rng;

    fn is_sorted<T>(mut input: impl Iterator<Item = T>, ascending: bool) -> bool
    where
        T: PartialOrd,
    {
        let mut current: Option<T> = None;
        for t in input.by_ref() {
            match current {
                None => current = Some(t),
                Some(prev) => match ascending {
                    true if prev > t => return false,
                    false if prev < t => return false,
                    _ => current = Some(t),
                },
            }
        }
        true
    }
    #[test]
    fn stack_sort_with_aux_stack_test_1() {
        let v = mv!(2);
        let v = sort_with_aux_stack(v, true);
        assert_eq!(v, mv![2]);

        let v = mv!(2, 1);
        let v = sort_with_aux_stack(v, true);
        assert_eq!(v, mv![1, 2]);

        let v = mv!(2, 1, 3);
        let v = sort_with_aux_stack(v, true);
        assert_eq!(v, mv![1, 2, 3]);

        let v = mv!(2, 1, 1, 2);
        let v = sort_with_aux_stack(v, true);
        assert_eq!(v, mv![1, 1, 2, 2]);

        let v = mv!(2);
        let v = sort_with_aux_stack(v, false);
        assert_eq!(v, mv![2]);

        let v = mv!(2, 1);
        let v = sort_with_aux_stack(v, false);
        assert_eq!(v, mv![2, 1]);
        let v = mv!(2, 1, 3);
        let v = sort_with_aux_stack(v, false);
        assert_eq!(v, mv![3, 2, 1]);

        let v = mv!(2, 1, 1, 2);
        let v = sort_with_aux_stack(v, false);
        assert_eq!(v, mv![2, 2, 1, 1]);

        let mut runs = 100;
        loop {
            let mut elems: [u16; 128] = [0; 128];
            rand::thread_rng().fill(&mut elems);
            let v = MiniVec::from_iter(elems);
            let v = sort_with_aux_stack(v, true);
            assert!(is_sorted(v.into_iter(), true));

            rand::thread_rng().fill(&mut elems);
            let v = MiniVec::from_iter(elems);
            let v = sort_with_aux_stack(v, false);
            assert!(is_sorted(v.into_iter(), false));

            runs -= 1;
            if runs == 0 {
                break;
            }
        }
    }

    #[test]
    fn stack_sort_test_1() {
        let v = mv!(2);
        let v = sort(v, true);
        assert_eq!(v, mv![2]);

        let v = mv!(2, 1);
        let v = sort(v, true);
        assert_eq!(v, mv![1, 2]);

        let v = mv!(2, 1, 3);
        let v = sort(v, true);
        assert_eq!(v, mv![1, 2, 3]);

        let v = mv!(2, 1, 1, 2);
        let v = sort(v, true);
        assert_eq!(v, mv![1, 1, 2, 2]);

        let mut runs = 10;
        loop {
            let mut elems: [u16; 128] = [0; 128];
            rand::thread_rng().fill(&mut elems);
            let v = MiniVec::from_iter(elems);
            let v = sort(v, true);
            assert!(is_sorted(v.into_iter(), true));

            rand::thread_rng().fill(&mut elems);
            let v = MiniVec::from_iter(elems);
            let v = sort(v, false);
            assert!(is_sorted(v.into_iter(), false));

            runs -= 1;
            if runs == 0 {
                break;
            }
        }
    }
}
