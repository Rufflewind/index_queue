use std::collections::VecDeque;
use super::*;

#[derive(Debug)]
enum Action {
    IsEmpty(),
    Contains(usize),
    Remove(usize),
    PushBack(usize),
    PopFront(),
}

#[derive(Debug, PartialEq, Eq)]
enum Output {
    Bool(bool),
    OptionUsize(Option<usize>),
}

trait State {
    fn perform(&mut self, action: Action) -> Output;
}

macro_rules! test {
    ($state1:expr, $state2:expr, $action:expr) => {
        assert_eq!($state1.perform($action),
                   $state2.perform($action),
                   "{:?}", $action)
    };
}

impl State for IndexQueue {
    fn perform(&mut self, action: Action) -> Output {
        use self::Action::*;
        use self::Output::*;
        match action {
            IsEmpty() => Bool(self.is_empty()),
            Contains(i) => Bool(self.contains(i)),
            Remove(i) => Bool(self.remove(i)),
            PushBack(i) => Bool(self.push_back(i)),
            PopFront() => OptionUsize(self.pop_front()),
        }
    }
}

impl State for VecDeque<usize> {
    fn perform(&mut self, action: Action) -> Output {
        use self::Action::*;
        use self::Output::*;
        match action {
            IsEmpty() => Bool(self.is_empty()),
            Contains(i) => Bool(self.contains(&i)),
            Remove(i) => Bool({
                let mut found = false;
                self.retain(|&j| {
                    if i == j {
                        found = true;
                        false
                    } else {
                        true
                    }
                });
                found
            }),
            PushBack(i) => Bool({
                if self.contains(&i) {
                    false
                } else {
                    self.push_back(i);
                    true
                }
            }),
            PopFront() => OptionUsize(self.pop_front()),
        }
    }
}

#[test]
fn main() {
    let ref mut this = IndexQueue::default();
    let ref mut model = VecDeque::default();

    test!(this, model, Action::IsEmpty());
    for i in 0 .. 32 {
        test!(this, model, Action::Contains(i));
    }

    let jss: &[&[usize]] = &[&[],
                             &[1],
                             &[3, 4],
                             &[5, 3, 6, 4, 1],
                             &[5, 3, 6, 4, 1,
                               2, 0, 1, 3, 4, 2, 6, 3, 6, 3, 10,
                               2]];
    for &js in jss {
        for &i in &[2, 0, 1, 3, 4, 2, 6, 3, 6, 3, 10] {
            test!(this, model, Action::PushBack(i));
            test!(this, model, Action::IsEmpty());
            for i in 0 .. 32 {
                test!(this, model, Action::Contains(i));
            }
        }

        for &j in js {
            test!(this, model, Action::Remove(j));
        }

        for _ in 0 .. 20 {
            test!(this, model, Action::PopFront());
            test!(this, model, Action::IsEmpty());
            for i in 0 .. 32 {
                test!(this, model, Action::Contains(i));
            }
        }
    }
}
