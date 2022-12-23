use ako::Ako;

fn main() {
    // TODO: read file as the input program
    let program = "a = 1".to_string();

    let mut ako = Ako::new(program);
    let err = ako.compile();
    if err.is_err() {
        println!("Syntax error for the input {:?}", err);
    }
}
