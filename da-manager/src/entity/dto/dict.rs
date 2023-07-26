use crate::entity::sys_entity::SysDict;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SysDictDTO {
    #[serde(flatten)]
    pub inner: SysDict,
}

impl From<SysDict> for SysDictDTO {
    fn from(arg: SysDict) -> Self {
        Self { inner: arg }
    }
}

impl SysDictDTO {}