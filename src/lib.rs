use gstd_fluent::{
    self as builder,
    gstd::{self, prelude::*, ActorId},
};

#[derive(Debug, Encode, Decode)]
#[codec(crate = gstd::codec)]
struct Output {
    a: i32,
    b: Option<bool>,
}

pub async fn f() {
    builder::send(ActorId::zero(), ())
        .with_delay(42)
        .with_gas_limit(61)
        .execute()
        .unwrap();

    let ret = builder::send(ActorId::zero(), Output { a: 42, b: None })
        .for_reply_as::<Output>()
        .execute()
        .unwrap()
        .await
        .unwrap();
}
