use crate::compiler::ast::{Program, Stmt};
use crate::aly::get_runtime;

pub struct Evaluator;

impl Evaluator {
    pub fn eval_program(program: &Program) -> i32 {
        let mut ctrl = 0;
        for stmt in &program.stmts {
            ctrl = Evaluator::eval_stmt(stmt);
            if ctrl != 0 {
                break;
            }
        }
        ctrl
    }

    fn eval_stmt(stmt: &Stmt) -> i32 {
        use Stmt::*;
        match stmt {
            Let { name, init } => {
                let value = if let Some(init_expr) = init {
                    get_runtime().eval_expr(init_expr)
                } else {
                    ValueData::None
                };
                get_runtime().create_variable(name.clone(), value);
                0
            }
            Const { name, init } => {
                let value = get_runtime().eval_expr(init);
                get_runtime().create_constant(name.clone(), value);
                0
            }
            Return(opt_expr) => {
                if let Some(expr) = opt_expr {
                    get_runtime().return_value(get_runtime().eval_expr(expr));
                } else {
                    get_runtime().return_value(ValueData::None);
                }
                0
            }
            If {
                condition,
                then_body,
                elifs,
                else_body,
            } => {
                if get_runtime().eval_expr(condition) == ValueData::Bool(true) {
                    get_runtime().run_stmts(then_body);
                } else {
                    for (cond, body) in elifs.iter() {
                        if get_runtime().eval_expr(cond) == ValueData::Bool(true) {
                            get_runtime().run_stmts(body);
                            return 0;
                        }
                    }
                    if let Some(body) = else_body {
                        get_runtime().run_stmts(body);
                    }
                }
                0
            }
            While { condition, body } => {
                while get_runtime().eval_expr(condition) == ValueData::Bool(true) {
                    get_runtime().run_stmts(body);
                }
                0
            }
            For { init, condition, update, body } => {
                if let Some(init_stmt) = init {
                    get_runtime().run_stmts(&[*init_stmt]);
                }
                while condition.as_ref().map_or(true, |c| get_runtime().eval_expr(c) == ValueData::Bool(true)) {
                    get_runtime().run_stmts(body);
                    if let Some(update) = update {
                        get_runtime().run_stmts(&[*update]);
                    }
                }
                0
            }
            ForIn { var, iter, body } => {
                let iter_val = get_runtime().eval_expr(iter);
                let mut iterator = get_runtime().get_iterator(iter_val);
                while let Some(item) = iterator.next() {
                    get_runtime().set_var(var.clone(), item);
                    get_runtime().run_stmts(body);
                }
                0
            }
            ForOf { var, iter, body } => {
                let iter_val = get_runtime().eval_expr(iter);
                let mut iterator = get_runtime().get_iterator(iter_val);
                while let Some(item) = iterator.next() {
                    get_runtime().set_var(var.clone(), item);
                    get_runtime().run_stmts(body);
                }
                0
            }
            ForRange { var, start, end, body } => {
                let start_i = get_runtime().eval_expr(start).to_string(false).parse::<i32>().unwrap_or(0);
                let end_i = get_runtime().eval_expr(end).to_string(false).parse::<i32>().unwrap_or(0);
                for i in start_i..end_i {
                    get_runtime().set_var(var.clone(), ValueData::Int(i));
                    get_runtime().run_stmts(body);
                }
                0
            }
            Fun { name, params, body } => {
                get_runtime().define_function(name.clone(), params, body.clone());
                0
            }
            Match { scrutinee, arms } => {
                let value = get_runtime().eval_expr(scrutinee);
                // naive match: just execute first arm body
                if let Some(arm_body) = arms.get(0) {
                    get_runtime().run_stmts(arm_body);
                }
                0
            }
            Expr(_) => 0,
        }
    }
}