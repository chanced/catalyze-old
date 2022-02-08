use crate::{lang::Lang, File, Package};
use std::collections::HashMap;
use std::rc::Rc;
struct Ast<L: Lang> {
    targets: HashMap<String, Rc<File<L>>>,
    packages: HashMap<String, Rc<Package<L>>>,
}
