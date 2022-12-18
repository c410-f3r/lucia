/// Type that indicates the usage of the `protobuf` dependency.
#[derive(Debug, Default)]
pub struct Protobuf;

#[cfg(all(feature = "std", test))]
mod tests {
  _create_dnsn_test!(
    borsh,
    (BorshRequest, BorshResponse),
    Borsh as Borsh,
    ([3, 0, 0, 0, 102, 111, 111][..].into(), [3, 0, 0, 0, 98, 97, 114][..].into()),
    (BorshRequest { data: Foo { foo: "foo" } }, BorshResponse { data: Bar { bar: "bar".into() } }),
  );
}
