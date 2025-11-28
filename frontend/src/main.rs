use frontend::lexer::Lexer;

fn main() {
    let tokens = Lexer::new("&&").tokens();
    println!("{:#?}", tokens)
}
