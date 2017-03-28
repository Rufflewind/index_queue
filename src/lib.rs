mod inner;
#[cfg(test)]
mod tests;

use std::{fmt, mem};
use self::inner::{Link, Slot};

#[derive(Clone, Default)]
pub struct IndexQueue {
    // this basically a double-linked list where each item is an index,
    // placed in the vector at the same index as its own value
    //
    //  (front)               (node)               (back)
    // ends.next ---> ... <--- prev
    //                         next ---> ... <--- ends.prev
    slots: Vec<Slot>,
    ends: Slot,
}

impl IndexQueue {
    pub fn is_empty(&self) -> bool {
        self.ends.is_empty()
    }

    pub fn contains(&self, index: usize) -> bool {
        match self.slots.get(index) {
            Some(slot) => !slot.is_empty(),
            _ => false,
        }
    }

    pub fn remove(&mut self, index: usize) -> bool {
        let slot = match self.slots.get_mut(index) {
            None => return false,
            Some(slot) => mem::replace(slot, Slot::default()),
        };
        match slot.as_ref() {
            None => return false,
            Some(&Link { prev, next }) => {
                let ends = self.ends.unwrap_mut();
                if ends.prev != ends.next {
                    if index == ends.next {
                        ends.next = next;
                    } else {
                        self.slots[prev].unwrap_mut().next = next;
                    }
                    if index == ends.prev {
                        ends.prev = prev;
                    } else {
                        self.slots[next].unwrap_mut().prev = prev;
                    }
                    return true;
                }
            }
        }
        self.ends.clear();
        true
    }

    /// Push an index to the back of the queue if it does not already
    /// exist.  Returns whether the index did not already exist.
    pub fn push_back(&mut self, index: usize) -> bool {
        loop {
            match self.slots.get_mut(index) {
                Some(ref slot) if !slot.is_empty() => return false,
                Some(slot) => {
                    *slot = Slot::from(Link {
                        next: /* irrelevant */ 0,
                        prev: self.ends.as_ref().map(|link| link.prev)
                            .unwrap_or(/* anything but INVALID_INDEX */ 0),
                    });
                    break;
                }
                None => {}
            }
            self.slots.push(Slot::default())
        }
        match self.ends.as_mut_ref() {
            None => {}
            Some(ends) => {
                let back = mem::replace(&mut ends.prev, index);
                self.slots[back].unwrap_mut().next = index;
                return true;
            }
        }
        self.ends = Slot::from(Link { prev: index, next: index });
        true
    }

    pub fn pop_front(&mut self) -> Option<usize> {
        let (is_last, popped) = {
            let (is_last, popped, front) = match self.ends.as_mut_ref() {
                None => return None,
                Some(ends) => {
                    let is_last = ends.prev == ends.next;
                    let popped = ends.next;
                    (is_last, popped, &mut ends.next)
                }
            };
            let slot = &mut self.slots[*front];
            let next = slot.unwrap().next;
            slot.clear();
            if !is_last {
                *front = next;
            }
            (is_last, popped)
        };
        if is_last {
            self.ends.clear();
        }
        Some(popped)
    }
}

impl fmt::Debug for IndexQueue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut queue = self.clone();
        let mut items = Vec::new();
        while let Some(i) = queue.pop_front() {
            items.push(i);
        }
        f.debug_tuple("IndexQueue").field(&items).finish()
    }
}
