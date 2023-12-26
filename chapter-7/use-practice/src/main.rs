use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;

/*
use std::io;
use std::io::Write;
*/
use std::io::{self, Write};

use std::collections::*;

/*
fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
*/

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
