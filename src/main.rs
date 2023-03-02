
use pettyscript::read_ast;


fn main() -> Result<(), ()>{
    let input = include_str!("../example.pty");
    let ast = read_ast(input)?;
    pettyscript::vm::run_virtual_machine(&ast);
    Ok(())
}
