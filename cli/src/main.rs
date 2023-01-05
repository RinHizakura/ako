use ako::Ako;

fn main() {
    // TODO: read file as the input program
    let program = "let a = 3 ^ 4;".to_string();

    let mut ako = Ako::new(program);
    let err = ako.compile_and_run();
    if err.is_err() {
        println!("Syntax error for the input {:?}", err);
    }
}
