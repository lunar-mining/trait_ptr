use async_trait::async_trait;
use smol::Executor;
use std::sync::{Arc, Weak};

fn main() {
    println!("Hello, world!");
}

pub type MyTraitPtr = Arc<dyn MyTrait>;

#[async_trait]
pub trait MyTrait {
    async fn foo(&self) {}
}

pub struct Parent {
    child: MyTraitPtr,
}
