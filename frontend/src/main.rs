use frontend::lexer::Lexer;

fn main() {
    let tokens = Lexer::new(
        r#"
let name = '9';
"#,
    )
    .tokens();

    println!("{:#?}", tokens)
}
