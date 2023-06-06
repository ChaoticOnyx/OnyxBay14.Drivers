use crate::Sgl;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Typeface {
    object_id: u64,
}

impl Typeface {
    pub fn new(data: &'static [u8], sgl: &mut Sgl) -> Self {
        let object_id = sgl.create_typeface(data);

        Self { object_id }
    }

    pub fn new_from_raw(object_id: u64) -> Self {
        Self { object_id }
    }

    pub fn id(&self) -> u64 {
        self.object_id
    }
}
