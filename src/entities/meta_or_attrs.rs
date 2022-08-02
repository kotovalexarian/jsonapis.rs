use super::*;

impl Entity<'_> for MetaOrAttrs {}

pub type MetaOrAttrs = HashMap<String, Value>;
