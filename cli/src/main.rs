use ako::Ako;

fn main() {
    // TODO: read file as the input program
    let program = "1 + 1".to_string();

    let mut ako = Ako::new(program);
    if ako.compile().is_err() {
        println!("Syntax error for the input");
    }
}
