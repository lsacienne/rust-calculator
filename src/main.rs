use parsemath::parser::{Parser, ParseError};
use slint::SharedString;

use crate::parsemath::ast;

slint::include_modules!();
mod parsemath;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_weak = ui.as_weak();
    ui.on_validate(move || {
        let ui = ui_weak.unwrap();
        let expression_ui: String = ui.get_expression().to_string();
        match evaluate(expression_ui.clone()) {
            Ok(result) => {
                ui.set_result(SharedString::from(result.to_string()));
                ui.set_is_err(false);
            },
            Err(e) => {
                ui.set_result(SharedString::from(e.to_string()));
                ui.set_is_err(true);
            }
        }
    });
    ui.run()
}

fn evaluate(expr: String) -> Result<f64, ParseError> {
    let expr = expr.split_whitespace().collect::<String>();
    // remove whitespace chars
    let mut math_parser = Parser::new(&expr)?;
    let ast = math_parser.parse()?;
    println!("The generated AST is {:?}", ast);
    Ok(ast::eval(ast)?)
}