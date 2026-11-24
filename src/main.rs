mod vm;

use vm::VM;


fn main() -> Result<(), String> {
    let program = "++[>++<";
    println!("Output:");
    let mut vm = VM::new(program)?;
    vm.run()?;
    println!("\nMemory: {:?}", &vm.memory[0..5]);
    println!("Data pointer: {}", vm.data_pointer);
    println!("{:?}", vm.jump_table);
    Ok(())
}
