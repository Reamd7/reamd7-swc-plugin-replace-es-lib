use serde::Deserialize;
use swc_core::ecma::{
    ast::{ImportDecl, Pass, CallExpr, Callee, Expr, Lit, Str, Ident, Program},
    visit::{noop_visit_mut_type, visit_mut_pass, VisitMut, VisitMutWith},
};

pub struct TransformVisitor {
    pub config: Config,
}

impl Pass for TransformVisitor {
    fn process(&mut self, program: &mut Program) {
        program.visit_mut_with(self);
    }
}

impl VisitMut for TransformVisitor {
    noop_visit_mut_type!();

    // 这里要做的事情就是把 @tarslib/utils/es/xxx 替换成 @tarslib/utils/lib/xxx
    // @tarslib/utils 是 Config 中 定义的模块
    // es => lib 还是 lib => es 由 Config 中的 direction 决定
    // eg: 
    // import { a } from "@tarslib/utils/es/xxx"; => import { a } from "@tarslib/utils/lib/xxx";
    // import("@tarslib/utils/es/xxx"); => import("@tarslib/utils/lib/xxx")
    // import a from "@tarslib/utils/es/xxx"; => import a from "@tarslib/utils/lib/xxx";

    fn visit_mut_import_decl(&mut self, n: &mut ImportDecl) {
        n.visit_mut_children_with(self);

        // 检查导入路径是否包含目标模块
        for module in &self.config.target_module {
            if n.src.value.contains(module) {
                let src = n.src.value.to_string();
                let new_src = if self.config.direction == "es2lib" {
                    src.replace("/es/", "/lib/")
                } else {
                    src.replace("/lib/", "/es/")
                };
                n.src.value = new_src.into();
                break;
            }
        }
    }

    fn visit_mut_call_expr(&mut self, n: &mut CallExpr) {
        n.visit_mut_children_with(self);

        // 处理动态导入和 require
        match &n.callee {
            Callee::Import(_) => {
                if let Some(arg) = n.args.get(0) {
                    if let Expr::Lit(Lit::Str(Str { value, .. })) = &*arg.expr {
                        for module in &self.config.target_module {
                            if value.contains(module) {
                                let new_value = if self.config.direction == "es2lib" {
                                    value.replace("/es/", "/lib/")
                                } else {
                                    value.replace("/lib/", "/es/")
                                };
                                let new_expr = Box::new(Expr::Lit(Lit::Str(Str {
                                    value: new_value.into(),
                                    span: Default::default(),
                                    raw: None,
                                })));
                                n.args[0].expr = new_expr;
                                break;
                            }
                        }
                    }
                }
            }
            Callee::Expr(expr) => {
                if let Expr::Ident(Ident { sym, .. }) = &**expr {
                    if sym == "require" {
                        if let Some(arg) = n.args.get(0) {
                            if let Expr::Lit(Lit::Str(Str { value, .. })) = &*arg.expr {
                                for module in &self.config.target_module {
                                    if value.contains(module) {
                                        let new_value = if self.config.direction == "es2lib" {
                                            value.replace("/es/", "/lib/")
                                        } else {
                                            value.replace("/lib/", "/es/")
                                        };
                                        let new_expr = Box::new(Expr::Lit(Lit::Str(Str {
                                            value: new_value.into(),
                                            span: Default::default(),
                                            raw: None,
                                        })));
                                        n.args[0].expr = new_expr;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    #[serde(default = "get_default_target_module")]
    pub target_module: Vec<String>,

    #[serde(default = "get_default_direction")]
    pub direction: String,
}
/**
 * es2lib | lib2es
 */
fn get_default_direction() -> String {
    "es2lib".to_string()
}

fn get_default_target_module() -> Vec<String> {
    vec![]
}

pub fn replace_es_lib(config: Config) -> impl Pass + VisitMut {
    visit_mut_pass(TransformVisitor { config })
}
