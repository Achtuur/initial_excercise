use crate::type_check::SgType;

pub enum SgExpr {
    /// Variable reference
    Var(String),
    /// Function definition(?) name, argument, body
    Fun(String, Box<SgType>, Box<SgExpr>),
    /// Function application / call (function, argument)
    Call(Box<SgExpr>, Box<SgExpr>),
    /// Number
    Num(i32),
    /// Boolean
    Bool(bool),
}

impl SgExpr {
    pub fn sg_type(&self) -> SgType {
        match self {
            SgExpr::Var(_) => SgType::VarT,
            SgExpr::Num(_) => SgType::NumT,
            SgExpr::Bool(_) => SgType::BoolT,
            SgExpr::Fun(_, _, body) => body.sg_type(),
            SgExpr::Call(_, _) => todo!("Get return type of function matching this signature"),
        }
    }
}

pub enum ModuleExpr {
    Module(ModuleExpr),
    Import(String),
    VarDef(String, SgType, SgExpr),
}
// a bit ugly here, but we only want module declarations in a program
pub struct Module {
    pub name: String,
    pub body: Vec<ModuleExpr>
}
impl Module {
    pub(crate) fn imports(&self, module2: Module) -> bool {
        todo!()
    }
}

pub type SgMProgram = Vec<Module>;