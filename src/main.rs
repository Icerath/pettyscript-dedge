use pettyscript::read_ast;

fn main() -> Result<(), ()> {
    let input = std::fs::read_to_string("example.pty").unwrap();
    let ast = read_ast(&input)?;
    pettyscript::vm::run_virtual_machine(&ast);
    Ok(())
}
