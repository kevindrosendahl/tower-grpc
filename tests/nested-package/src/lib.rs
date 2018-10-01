extern crate bytes;
extern crate prost;
#[macro_use]
extern crate prost_derive;
extern crate tower_grpc;

pub mod mypackage {
    include!(concat!(env!("OUT_DIR"), "/mypackage.rs"));

    pub mod foo {
        include!(concat!(env!("OUT_DIR"), "/mypackage.foo.rs"));
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    #[test]
    fn types_are_present() {
        mem::size_of::<::mypackage::foo::FooRequest>();
    }
}

