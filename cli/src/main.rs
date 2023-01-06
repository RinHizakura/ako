use ako::Ako;

fn main() {
    // TODO: read file as the input program
        let program = "let a = 7 & 8;\n \
                       let b = 6 | 4;\n \
                       let c = 6 ^ 4;\n \
                       let d = 9 + 6;\n \
                       let e = 5 - 7;\n \
                       let f = 8 * 2;\n \
                       let g = 8 / 2;\n \
                       let h = 8 % 2;".to_string();

    let mut ako = Ako::new(program);
    let err = ako.compile_and_run();
    if err.is_err() {
        println!("Syntax error for the input {:?}", err);
    }
}
