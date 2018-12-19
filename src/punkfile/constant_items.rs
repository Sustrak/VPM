pub enum ConstantItems {
    ConstantInteger(i64),
    ConstantClass(usize),
    ConstantInfo(String),
    ConstantString(String),
    ConstantUnimplemented(u64),
    ConstantNameType {name: usize, desc: usize},
    ConstantField {class: usize, name_type: usize},
    ConstantMethod {class: usize, name_type: usize}
}