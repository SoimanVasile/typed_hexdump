use crate::struct_type::StructType;

pub struct FieldInfo{
    pub name: &'static str,
    pub type_field: StructType,
    pub offset: usize,
}
