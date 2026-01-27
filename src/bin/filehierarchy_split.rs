// This example demonstrates Rust's module system with nested modules.
// Note: In a real project with multiple files, you would use `mod my;` to
// include a file named `my.rs` or `my/mod.rs`. Since this is a standalone
// binary example, we define the modules inline.
//
// For the original multi-file example, see the `filehierarchy/` directory.

mod my {
    // Private module - cannot be accessed from outside `my`
    mod inaccessible {
        #[allow(dead_code)]
        pub fn public_function() {
            println!("called `my::inaccessible::public_function()`");
        }
    }

    // Public nested module
    pub mod nested {
        pub fn function() {
            println!("called `my::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my::nested::private_function()`");
        }
    }

    pub fn function() {
        println!("called `my::function()`");
    }

    fn private_function() {
        println!("called `my::private_function()`");
    }

    pub fn indirect_access() {
        print!("called `my::indirect_access()`, that\n> ");
        private_function();
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}
