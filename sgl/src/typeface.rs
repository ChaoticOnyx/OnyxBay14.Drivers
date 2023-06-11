#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Typeface {
    object_id: u64,
}

impl Typeface {
    pub(crate) fn new_from_id(object_id: u64) -> Self {
        Self { object_id }
    }

    pub fn id(&self) -> u64 {
        self.object_id
    }
}
