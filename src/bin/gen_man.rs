// create a man page for solidpython_config using the man crate
use man::prelude::*;
fn main(){
    let page = Manual::new("solidpython_config").about("a crate that generates configuration based solidpython template")
    .author(Author::new("Josh Ward"))
    .render();
    println!("{}",page);
}