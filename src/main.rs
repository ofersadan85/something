mod prelude;
use prelude::*;

fn main() -> Result<()> {
    prelude()?;
    println!("{}", "Hello, world!".red().strikethrough());
    debug!("This is a debug message");
    Ok(())
}
