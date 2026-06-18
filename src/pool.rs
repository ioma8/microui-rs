pub type Id = u32;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PoolItem {
    pub id: Id,
    pub last_update: i32,
}
