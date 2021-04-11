use std::env;
use std::io::{Error, ErrorKind, Result};

use steganography::run;

fn main() -> Result<()> {
    let v_args : Vec<String> = env::args().collect();

    if v_args.len() < 2 {
        return Err(Error::new(ErrorKind::InvalidInput, "Pas assez d'arguments"));
    }

    run(v_args[1].as_str(), v_args[2].as_str())?;

    Ok(())
}
