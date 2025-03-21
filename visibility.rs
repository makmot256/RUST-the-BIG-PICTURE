// Visibility in Rust

// Public items are visible from anywhere, public items in parent modules are visible in child modules
pub mod public_items {
    pub fn public_function() {
        println!("public_function");
    }
}

// Private items are visible only in the same module
mod private_items {
    fn private_function() {
        println!("private_function");
    }
}

// Public items can be re-exported
pub use public_items::public_function as public_function;

// Private items cannot be re-exported
// use private_items::private_function; // this would cause a compile error

// Items in a parent module are visible in child modules
pub mod parent {
    pub mod child {
        pub fn child_function() {
            println!("child_function");
        }
    }
}

// Items in a child module are not visible in parent modules
// parent::child::child_function(); // this would cause a compile error

// Items in a sibling module are not visible in another sibling module
pub mod sibling1 {
    pub fn sibling1_function() {
        println!("sibling1_function");
    }
}

pub mod sibling2 {
    // sibling1::sibling1_function(); // this would cause a compile error
}

// Items in a module can be re-exported
pub use sibling1::sibling1_function as sibling1_function;

// Public items can be imported from another module
mod other_module {
    pub fn other_module_function() {
        println!("other_module_function");
    }
}

use other_module::other_module_function as other_module_function;

// Private items cannot be imported from another module
// use other_module::private_function; // this would cause a compile error

// Public items can be imported from another crate
extern crate rand;

use rand::random;

// Private items cannot be imported from another crate
// use rand::private_function; // this would cause a compile error

// Items can be imported with a shorter name
use rand::random as random_number;

// Items can be imported with a shorter name and given a new name
use rand::random as random_number_new_name;

// Items can be imported with a shorter name and given a new name and a namespace
use rand::random as random_number_new_name_in_namespace {
    random_number_new_name as random_number_new_name_in_namespace,
};

// Items can be imported with a shorter name and given a new name and a namespace and a version
#[cfg(feature = "nightly")]
use rand::random as random_number_new_name_in_namespace_with_version {
    random_number_new_name as random_number_new_name_in_namespace_with_version,
};

// Items can be imported with a shorter name and given a new name and a namespace and a version and a feature
#[cfg(feature = "nightly")]
#[cfg(feature = "serde")]
use rand::random as random_number_new_name_in_namespace_with_version_with_feature {
    random_number_new_name as random_number_new_name_in_namespace_with_version_with_feature,
};

fn main() {
    public_function();
    parent::child::child_function();
    sibling1_function();
    other_module_function();
    random_number();
    random_number_new_name();
    random_number_new_name_in_namespace::random_number_new_name_in_namespace();
    #[cfg(feature = "nightly")]
    random_number_new_name_in_namespace_with_version::random_number_new_name_in_namespace_with_version();
    #[cfg(feature = "nightly")]
    #[cfg(feature = "serde")]
    random_number_new_name_in_namespace_with_version_with_feature::random_number_new_name_in_namespace_with_version_with_feature();
}
