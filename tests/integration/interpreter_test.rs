use befunge_rust::{
    interpreter::{Interpreter, Direction},
    world::World,
};
use std::error::Error;
use std::io::BufReader;

#[test]
fn hello_world_program() -> Result<(), Box<dyn Error>> {
    let src = ">              v\nv  ,,,,,\"Hello\"<\n>48*,          v\nv,,,,,,\"World!\"<\n>25*,@";
    let read = Vec::new();
    let mut buf_read = BufReader::new(&read[..]);
    let mut write = Vec::new();
    let mut world = World::from_source_string(src);
    {
        let mut interpreter = Interpreter::new(
            &mut world,
            0,
            0,
            Direction::Right,
            &mut buf_read,
            &mut write,
            false,
        );
        interpreter.run()?;
    }
    assert_eq!(String::from_utf8_lossy(&write[..]), "Hello World!\n");
    Ok(())
}

#[test]
fn factorial_calculator() -> Result<(), Box<dyn Error>> {
    let read = Vec::new();
    let mut buf_read = BufReader::new(&read[..]);
    let mut write = Vec::new();
    let mut world = World::from_source_string("5 100p:v\nv *g00:_00g.@\n>00p1-:^");
    {
        let mut interpreter = Interpreter::new(
            &mut world,
            0,
            0,
            Direction::Right,
            &mut buf_read,
            &mut write,
            false,
        );
        interpreter.run()?;
    }
    assert_eq!(String::from_utf8_lossy(&write[..]), "120 ");
    Ok(())
}

#[test]
fn debug_mode() -> Result<(), Box<dyn Error>> {
    let read = Vec::from("\n\n".as_bytes());
    let mut buf_read = BufReader::new(&read[..]);
    let mut write = Vec::new();
    let mut world = World::from_source_string("65@");  // ASCII value for 'A'
    {
        let mut interpreter = Interpreter::new(
            &mut world,
            0,
            0,
            Direction::Right,
            &mut buf_read,
            &mut write,
            true,
        );
        interpreter.run()?;
    }

    let output = String::from_utf8_lossy(&write);
    assert!(output.contains("=== Step Debug Info ==="));
    assert!(output.contains("Stack: [0x41 ('A')]"));
    Ok(())
}