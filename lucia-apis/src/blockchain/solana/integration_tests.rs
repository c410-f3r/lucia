use crate::blockchain::solana::{
  AccountEncoding, CommitmenOptDataSliceOptEncodingMand, Commitment, CommitmentOptEncoding,
  CommitmentOptEncodingOpt, DataSlice, Filter, GenericTransaction, GetProgramAccountsReqParams,
  InstructionJsonParsedInfo, Memcmp, MemcmpEncodedBytes, MessageInput, MintOrProgramId, Solana,
  SolanaAddressHash, TransactionEncoding, TransactionInput,
};
use alloc::vec::Vec;
use ed25519_dalek::Keypair;
use lucia::{
  dnsn::SerdeJson,
  misc::CommonParams,
  network::{http, ws, Transport},
};

const ALICE_PUBLIC_KEY: SolanaAddressHash = [
  72, 221, 15, 10, 15, 203, 187, 109, 166, 124, 138, 38, 199, 74, 146, 72, 63, 245, 197, 247, 201,
  170, 164, 254, 147, 227, 243, 91, 101, 49, 105, 158,
];
const ALICE_SECRET_KEY: SolanaAddressHash = [
  30, 33, 76, 185, 72, 178, 196, 11, 231, 116, 8, 208, 127, 141, 89, 39, 217, 222, 1, 167, 32, 61,
  224, 135, 200, 132, 58, 174, 231, 165, 32, 132,
];
const BOB_PUBLIC_KEY: SolanaAddressHash = [
  24, 147, 209, 196, 197, 185, 156, 48, 170, 96, 192, 119, 193, 150, 129, 12, 221, 102, 119, 84,
  33, 221, 67, 224, 185, 107, 130, 157, 207, 85, 161, 30,
];
const TO_NORMAL_ACCOUNT: &str = "FiuQrMbFUYka1Goec4wdhoiNq3Ms99cxGrW8JWsWfPnJ";
const TO_SOL_TOKEN_ACCOUNT: &str = "CDqKzghiixHryqny9r8RPJzYfg3hiiF7e8JecsF6fuJw";
const TO_SOL_TOKEN_MINT: &str = "So11111111111111111111111111111111111111112";
const TOKEN_PROGRAM: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

_create_http_test!(http(), get_account_info, |rm, trans| async {
  let req = rm.get_account_info(
    TO_NORMAL_ACCOUNT,
    Some(CommitmenOptDataSliceOptEncodingMand {
      commitment: None,
      data_slice: None,
      encoding: AccountEncoding::JsonParsed,
    }),
  );
  let _ = trans.send_retrieve_and_decode_one(rm, &req, ()).await.unwrap().result.value.unwrap();
});

_create_http_test!(http(), get_balance, |rm, trans| async {
  let req = rm.get_balance(TO_NORMAL_ACCOUNT, None);
  let _ = trans.send_retrieve_and_decode_one(rm, &req, ()).await.unwrap();
});

_create_http_test!(http(), get_block_height, |rm, trans| async {
  let req = rm.get_block_height(None, None);
  let _ = trans.send_retrieve_and_decode_one(rm, &req, ()).await.unwrap();
});

#[cfg(feature = "solana-program")]
_create_http_test!(http(), get_fee_for_message, |rm, trans| async {
  let mut buffer = Vec::new();
  let from_keypair = Keypair::from_bytes(&alice_keypair()[..]).unwrap();
  let from_public_key = from_keypair.public.to_bytes();
  let req = rm.get_latest_blockhash(None);
  let blockhash =
    trans.send_retrieve_and_decode_one(rm, &req, ()).await.unwrap().result.value.blockhash;
  let message = transfer_message(blockhash, from_public_key);
  let req = rm.get_fee_for_message(&mut buffer, &message, None).unwrap();
  assert_eq!(
    trans.send_retrieve_and_decode_one(rm, &req, ()).await.unwrap().result.value.unwrap(),
    5000
  );
});

_create_http_test!(http(), get_minimum_balance_for_rent_exemption, |rm, trans| async {
  let req = rm.get_minimum_balance_for_rent_exemption(100, None);
  let _ = trans.send_retrieve_and_decode_one(rm, &req, ()).await.unwrap();
});

_create_http_test!(http(), get_multiple_accounts, |rm, trans| async {
  let req = rm.get_multiple_accounts(&[TO_NORMAL_ACCOUNT, TO_SOL_TOKEN_ACCOUNT], None);
  let _ = trans.send_retrieve_and_decode_one(rm, &req, ()).await.unwrap();
});

_create_http_test!(http(), get_program_accounts, |rm, trans| async {
  let req = rm.get_program_accounts(
    TOKEN_PROGRAM,
    Some(GetProgramAccountsReqParams {
      commitment: None,
      data_slice: Some(DataSlice { length: 32, offset: 0 }),
      encoding: Some(AccountEncoding::Base64),
      filters: Some(&[
        Filter::DataSize(165),
        Filter::Memcmp(Memcmp { bytes: MemcmpEncodedBytes::Base58(TO_NORMAL_ACCOUNT), offset: 32 }),
      ]),
      min_context_slot: None,
    }),
  );
  assert_eq!(trans.send_retrieve_and_decode_one(rm, &req, ()).await.unwrap().result.len(), 1);
});

_create_http_test!(http(), get_slot, |rm, trans| async {
  let req = rm.get_slot(None);
  let _ = trans.send_retrieve_and_decode_one(rm, &req, ()).await.unwrap();
});

_create_http_test!(http(), get_token_account_balance, |rm, trans| async {
  let req = rm.get_token_account_balance(TO_SOL_TOKEN_ACCOUNT, None);
  let _ = trans.send_retrieve_and_decode_one(rm, &req, ()).await.unwrap();
});

_create_http_test!(http(), get_token_accounts_by_owner, |rm, trans| async {
  let req = rm.get_token_accounts_by_owner::<16, _>(
    TO_NORMAL_ACCOUNT,
    MintOrProgramId::Mint(TO_SOL_TOKEN_MINT),
    Some(CommitmenOptDataSliceOptEncodingMand {
      commitment: None,
      data_slice: None,
      encoding: AccountEncoding::JsonParsed,
    }),
  );
  let res = trans.send_retrieve_and_decode_one(rm, &req, ()).await.unwrap();
  let _ = res.result.value[0].pubkey;
});

_create_http_test!(http(), get_version, |rm, trans| async {
  let req = rm.get_version();
  let res = trans.send_retrieve_and_decode_one(rm, &req, ()).await.unwrap();
  let _ = res.result.feature_set;
});

#[cfg(feature = "solana-program")]
_create_http_test!(
  http(),
  http_get_latest_blockhash_send_transaction_and_get_transaction,
  |rm, trans| async {
    let from_keypair = Keypair::from_bytes(&alice_keypair()[..]).unwrap();
    let from_public_key = from_keypair.public.to_bytes();
    let req = rm.get_latest_blockhash(None);
    let blockhash =
      trans.send_retrieve_and_decode_one(rm, &req, ()).await.unwrap().result.value.blockhash;
    let message = transfer_message(blockhash, from_public_key);
    let mut buffer = Vec::new();
    let tx = TransactionInput::new(&mut buffer, blockhash, message, &[from_keypair]).unwrap();
    buffer.clear();
    let req = rm.send_transaction(&mut buffer, None, &tx).unwrap();
    let tx_hash = trans.send_retrieve_and_decode_one(rm, &req, ()).await.unwrap().result;
    assert!(Solana::confirm_transaction(<_>::default(), &tx_hash, rm, trans).await.unwrap());

    let req = rm.get_transaction(
      tx_hash.as_str(),
      Some(CommitmentOptEncodingOpt {
        commitment: Some(Commitment::Finalized),
        encoding: Some(TransactionEncoding::Base64),
      }),
    );
    let _ = trans.send_retrieve_and_decode_one(rm, &req, ()).await.unwrap().result.transaction;

    let req = rm.get_transaction(
      tx_hash.as_str(),
      Some(CommitmentOptEncodingOpt {
        commitment: Some(Commitment::Finalized),
        encoding: Some(TransactionEncoding::Json),
      }),
    );
    let _ = trans.send_retrieve_and_decode_one(rm, &req, ()).await.unwrap().result.transaction;

    let req = rm.get_transaction(
      tx_hash.as_str(),
      Some(CommitmentOptEncodingOpt {
        commitment: Some(Commitment::Finalized),
        encoding: Some(TransactionEncoding::JsonParsed),
      }),
    );
    let tx = trans.send_retrieve_and_decode_one(rm, &req, ()).await.unwrap().result.transaction;
    if !matches!(
      generic_tx_parsed_instruction(&tx, 0).unwrap(),
      InstructionJsonParsedInfo::TransferInstruction(..)
    ) {
      panic!();
    }
  }
);

_create_http_test!(http(), http_reqs_with_array, |rm, trans| async {
  let first = rm.get_balance(TO_NORMAL_ACCOUNT, None);
  let second = rm.get_balance(TO_NORMAL_ACCOUNT, None);
  let mut buffer = Vec::new();
  let _ = trans.send_retrieve_and_decode_many(&mut buffer, rm, &[first, second], ()).await.unwrap();
});

_create_tokio_tungstenite_test!(
  ws(),
  account_subscribe,
  (account_unsubscribe),
  |rm, trans| async {
    let req = rm.account_subscribe(
      TO_NORMAL_ACCOUNT,
      Some(CommitmentOptEncoding { commitment: None, encoding: AccountEncoding::JsonParsed }),
    );
    [trans.send_retrieve_and_decode_one(rm, &req, ()).await.unwrap().result]
  }
);

_create_tokio_tungstenite_test!(ws(), root_subscribe, (root_unsubscribe), |rm, trans| async {
  let req = rm.root_subscribe();
  [trans.send_retrieve_and_decode_one(rm, &req, ()).await.unwrap().result]
});

_create_tokio_tungstenite_test!(ws(), slot_subscribe, (slot_unsubscribe), |rm, trans| async {
  let req = rm.slot_subscribe();
  [trans.send_retrieve_and_decode_one(rm, &req, ()).await.unwrap().result]
});

_create_tokio_tungstenite_test!(
  ws(),
  ws_reqs_with_array,
  (account_unsubscribe, account_unsubscribe),
  |rm, trans| async {
    let first = rm.account_subscribe(
      TO_NORMAL_ACCOUNT,
      Some(CommitmentOptEncoding { commitment: None, encoding: AccountEncoding::JsonParsed }),
    );
    let second = rm.account_subscribe(
      TO_NORMAL_ACCOUNT,
      Some(CommitmentOptEncoding { commitment: None, encoding: AccountEncoding::JsonParsed }),
    );
    let mut buffer = Vec::new();
    trans.send_retrieve_and_decode_many(&mut buffer, rm, &[first, second], ()).await.unwrap();
    [buffer[0].result, buffer[1].result]
  }
);

fn alice_keypair() -> [u8; 64] {
  let mut array = [0; 64];
  array[..32].copy_from_slice(&ALICE_SECRET_KEY);
  array[32..].copy_from_slice(&ALICE_PUBLIC_KEY);
  array
}

fn http() -> (CommonParams<http::ReqParams, ()>, SerdeJson) {
  (
    CommonParams::new(http::ReqParams::from_origin("http://localhost:8899").unwrap(), ()),
    SerdeJson::default(),
  )
}

fn generic_tx_parsed_instruction<'tx>(
  tx: &'tx GenericTransaction,
  idx: usize,
) -> Option<&'tx InstructionJsonParsedInfo> {
  if let GenericTransaction::JsonParsed(ref tx_json) = tx {
    Some(&tx_json.message.instructions.get(idx)?.parsed.as_ref()?.info)
  } else {
    None
  }
}

#[cfg(feature = "solana-program")]
fn transfer_message(blockhash: [u8; 32], from_public_key: [u8; 32]) -> MessageInput {
  let transfer = solana_program::system_instruction::transfer(
    &from_public_key.into(),
    &BOB_PUBLIC_KEY.into(),
    100000000,
  )
  .try_into()
  .unwrap();
  MessageInput::with_params(&[transfer], Some(from_public_key), blockhash).unwrap()
}

fn ws() -> (CommonParams<ws::ReqParams, ()>, SerdeJson) {
  (
    CommonParams::new(ws::ReqParams::from_origin("ws://localhost:8900").unwrap(), ()),
    SerdeJson::default(),
  )
}
