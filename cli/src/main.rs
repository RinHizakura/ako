use ako::Ako;
use rustyline::Editor;

fn print_version() {
    println!("ako 0.1.0");
}

fn repl() {
    let mut rl = Editor::<()>::new().expect("Fail to start rustyline");
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let mut ako = Ako::new(line);
                ako.compile();
            }
            Err(err) => panic!("Readline error: {:?}", err),
        }
    }
}

fn main() {
    print_version();
    repl();
}
