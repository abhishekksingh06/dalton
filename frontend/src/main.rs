use frontend::lexer::Lexer;

fn main() {
    let tokens = Lexer::new("let name = 10;").tokens();
    println!("{:#?}", tokens)
}
