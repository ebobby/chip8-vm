mod chip8;

fn main() -> std::io::Result<()> {
    let mut vm = chip8::new();

    vm.reset();
    vm.load_rom(String::from("SCTEST.ch8"))?;
    vm.run_test();

    Ok(())
}
