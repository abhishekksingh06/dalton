use frontend::lexer::Lexer;

fn main() {
    let tokens = Lexer::new(
        r#"
let name = "हिन्दी\\n";
"#,
    )
    .tokens();

    println!("{:#?}", tokens)
}

