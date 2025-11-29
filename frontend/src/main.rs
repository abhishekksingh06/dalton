use frontend::lexer::Lexer;

fn main() {
    let tokens = Lexer::new(
        r#"
let name = 928_;
"#,
    )
    .tokens();

    println!("{:#?}", tokens)
}
