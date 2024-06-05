use std::{collections::HashMap, marker::PhantomData};

use crate::{expr::{ModuleExpr, SgExpr, SgMProgram}, label::ModuleLabel, scopegraph::{Scope, ScopeGraph}};
use crate::error::Result;

pub enum SgType {
    FunT(Box<SgType>, Box<SgType>),
    VarT,
    NumT,
    BoolT,
}

impl SgType {
}

type LCLbl = String; // strings for now
type LCData = SgExpr;

type TCResult = Result<(ScopeGraph<LCLbl, LCData>, SgType)>;


pub fn type_check(expr: &LCData) -> TCResult {
    let mut sg = ScopeGraph::<LCLbl, LCData>::empty();
    // not sure if this argument here should be expr
    let global_scope = sg.new_scope(expr);
    type_of_expr(sg, expr, &global_scope)
}

pub fn type_of_expr(sg: ScopeGraph<LCLbl, LCData>, expr: &LCData, scope: &Scope) -> TCResult {
    let path_ok = "P*"; // expression must be reachable from current scope (which is just global scope)
    let data_ok = todo!("check if queried data is equal to expr?");
    let label_order = "D < P"; // i guess?
    let query_result = sg.query(&scope, path_ok, data_ok, label_order);
    // query_result is now a list of DATA that can be `expr`
    if query_result.is_empty() {
        return Err("no expression found".into());
    }
    // assume first is data we're looking for?
    // for function application type expressions, query the function definition
    match query_result[0] {
        SgExpr::Call(fun, _) => {
            type_of_expr(sg, &fun, scope)
        }
        _ => Ok((sg, query_result[0].sg_type()))
    }
}


// WITH MODULES

type MDLbl = ModuleLabel; // strings for now
type MDData = ModuleExpr;

type TCMResult = Result<ScopeGraph<MDLbl, MDData>>;

pub fn programOk(program: SgMProgram) -> TCMResult {
    // assumption: global scope is either empty, or its own module
    // based type Program = [Module]

    // construct scope graph from prog
    let mut sg = ScopeGraph::<MDLbl, MDData>::empty();
    // scopes are stored as a <module_name, scope> hashmap
    let mut scopes = HashMap::new(); 
    // add all modules as a new scope
    for module in program {
        let scope = sg.new_scope(&ModuleExpr::Module(module)); //im just gonna assume this adds MDDATA into the scope
        scopes.insert(module.name, scope);
    }

    // add edges between modules if there are imports?
    // this is a really slow naive implementation
    for module in program {
        for module2 in program {
            if module.imports(module2) {
                let m1_scope = scopes.get(&module.name).unwrap();
                let m2_scope = scopes.get(&module2.name).unwrap();
                sg.add_edge(m1_scope, ModuleLabel::Import, m2_scope);
            }
        }
    }

    // program is ok if all modules types are ok
    for module in program {
        let scope = scopes.get(&module.name).unwrap();
        let body = module.body;
        for expr in body {
            if let Err(_) = type_of_expr_module(sg, &expr, scope) {
                return Err("module type error".into());
            }
        }
    }
    Ok(sg)

}

pub fn type_of_expr_module(sg: ScopeGraph<MDLbl, MDData>, expr: &MDData, scope: &Scope) -> TCMResult {
    let path_ok = "P*I*"; // expression must be reachable from current scope
    let data_ok = todo!("check if queried data is equal to expr?");
    let label_order = "D < I, D < P, I < P"; // i guess?
    let query_result = sg.query(scope, path_ok, data_ok, label_order);
    // query_result is now a list of DATA that can be `expr`
    if query_result.is_empty() {
        return Err("no expression found".into());
    }
    // assume first is data we're looking for?
    // for function application type expressions, query the function definition
    match query_result[0] {
        todo!("match function call") => type_of_expr_module(sg, fun_expr, scope),
        _ => Ok((sg, query_result[0].sg_type()))
    }
}

