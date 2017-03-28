const INVALID_INDEX: usize = !0;

#[derive(Clone, Copy, Debug)]
pub struct Link {
    pub prev: usize,
    pub next: usize,
}

/// Slots can be either filled or empty.
///
///   - An empty slot is created using `Default`.
///     An existing slot can be made empty using `clear`.
///   - A filled slot is created using `From`.
///
/// Internally, empty slots are represented by setting `prev` to
/// `INVALID_INDEX`.
#[derive(Clone, Copy, Debug)]
pub struct Slot(Link);

impl Default for Slot {
    fn default() -> Self {
        Slot(Link {
            prev: INVALID_INDEX,
            next: /* irrelevant */ 0,
        })
    }
}

impl From<Link> for Slot {
    fn from(value: Link) -> Self {
        debug_assert!(value.prev != INVALID_INDEX);
        Slot(value)
    }
}

impl Slot {
    pub fn is_empty(&self) -> bool {
        self.0.prev == INVALID_INDEX
    }

    pub fn clear(&mut self) {
        self.0.prev = INVALID_INDEX;
    }

    pub fn as_ref(&self) -> Option<&Link> {
        if self.is_empty() {
            None
        } else {
            Some(&self.0)
        }
    }

    pub fn as_mut_ref(&mut self) -> Option<&mut Link> {
        if self.is_empty() {
            None
        } else {
            Some(&mut self.0)
        }
    }

    pub fn unwrap(&self) -> &Link {
        debug_assert!(!self.is_empty());
        &self.0
    }

    pub fn unwrap_mut(&mut self) -> &mut Link {
        debug_assert!(!self.is_empty());
        &mut self.0
    }
}
