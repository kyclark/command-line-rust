#[derive(Clone, Copy)]
pub enum Owner {
    User,
    Group,
    Other,
}

impl Owner {
    pub fn masks(self) -> [u32; 3] {
        match self {
            Self::User => [0o400, 0o200, 0o100],
            Self::Group => [0o040, 0o020, 0o010],
            Self::Other => [0o004, 0o002, 0o001],
        }
    }
}
