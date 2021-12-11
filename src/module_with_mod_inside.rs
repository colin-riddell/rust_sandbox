// module_with_mod_inside is a module


pub mod foo { // the module foo is defined inside the module_with_module_inside
    pub fn bar() { // bar is within foo
        println!("called from within module module_with_mod_inside within module foo created with mod keyword")
    }
}