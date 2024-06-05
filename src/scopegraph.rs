use std::marker::PhantomData;

pub struct Scope;

pub struct ScopeGraph<LBL, DATA> {
    pd: PhantomData<LBL>, // todo so it compiles
    pd2: PhantomData<DATA>,
}


impl<LBL, DATA> ScopeGraph<LBL, DATA> {
    pub fn empty() -> Self {
        todo!()
    }

    pub fn new_scope(&mut self, data: &DATA) -> Scope {
        todo!()
    }

    pub fn add_edge(&mut self, src: &Scope, lbl: LBL, tgt: &Scope) {
        todo!()
    }

    // path_ok and label_order are string so it compiles for now
    pub fn query(&mut self, src: &Scope, path_ok: &str, data_ok: fn(DATA) -> bool, label_order: &str) -> Vec<DATA> {
        todo!()
    }
}