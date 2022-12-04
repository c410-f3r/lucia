use crate::blockchain::solana::{
  AccountEncoding, AccountSubscribeConfig, Commitment, DataSlice, Filter, GenericTransaction,
  GetAccountInfoConfig, GetBlockConfig, GetProgramAccountsConfig, GetTokenAccountsByOwnerConfig,
  GetTransactionConfig, InstructionJsonParsedInfo, Memcmp, MemcmpEncodedBytes, MessageInput,
  MintOrProgramId, Solana, SolanaAddressHash, TransactionDetails, TransactionEncoding,
  TransactionInput,
};
use alloc::vec::Vec;
use ed25519_dalek::Keypair;
use lucia::{
  dnsn::SerdeJson,
  network::{transport::Transport, HttpParams, WsParams},
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
const WS_URL: &str = "ws://localhost:8900";

_create_http_test!(Solana::new(None), http(), get_account_info, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux
    .get_account_info()
    .data(
      TO_NORMAL_ACCOUNT,
      Some(GetAccountInfoConfig {
        commitment: None,
        data_slice: None,
        encoding: Some(AccountEncoding::JsonParsed),
      }),
    )
    .build();
  let _ = trans
    .send_retrieve_and_decode_contained(pkg, pkgs_aux)
    .await
    .unwrap()
    .result
    .unwrap()
    .value
    .unwrap();
});

_create_http_test!(Solana::new(None), http(), get_balance, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.get_balance().data(TO_NORMAL_ACCOUNT, None).build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(Solana::new(None), http(), get_block, |pkgs_aux, trans| async {
  let slot_pkg = &mut pkgs_aux.get_slot().data(None, None).build();
  let slot =
    trans.send_retrieve_and_decode_contained(slot_pkg, pkgs_aux).await.unwrap().result.unwrap();
  let get_block_pkg = &mut pkgs_aux
    .get_block()
    .data(
      slot,
      Some(GetBlockConfig {
        commitment: Some(Commitment::Finalized),
        encoding: Some(TransactionEncoding::JsonParsed),
        rewards: Some(true),
        transaction_details: Some(TransactionDetails::Full),
      }),
    )
    .build();
  let _ = trans.send_retrieve_and_decode_contained(get_block_pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(Solana::new(None), http(), get_block_height, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.get_block_height().data(Some(Commitment::Finalized), Some(2)).build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(Solana::new(None), http(), get_cluster_nodes, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.get_cluster_nodes().build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

#[cfg(feature = "solana-program")]
_create_http_test!(Solana::new(None), http(), get_fee_for_message, |pkgs_aux, trans| async {
  let from_keypair = Keypair::from_bytes(&alice_keypair()[..]).unwrap();
  let from_public_key = from_keypair.public.to_bytes();
  let get_latest_blockhash_pkg = &mut pkgs_aux.get_latest_blockhash().data(None, None).build();
  let blockhash = trans
    .send_retrieve_and_decode_contained(get_latest_blockhash_pkg, pkgs_aux)
    .await
    .unwrap()
    .result
    .unwrap()
    .value
    .blockhash;
  let message = transfer_message(blockhash, from_public_key);
  let get_fee_for_message_pkg =
    &mut pkgs_aux.get_fee_for_message().data(None, &message).unwrap().build();
  assert_eq!(
    trans
      .send_retrieve_and_decode_contained(get_fee_for_message_pkg, pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap()
      .value
      .unwrap(),
    5000
  );
});

_create_http_test!(
  Solana::new(None),
  http(),
  get_minimum_balance_for_rent_exemption,
  |pkgs_aux, trans| async {
    let pkg = &mut pkgs_aux.get_minimum_balance_for_rent_exemption().data(100, None).build();
    let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
  }
);

_create_http_test!(Solana::new(None), http(), get_multiple_accounts, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux
    .get_multiple_accounts()
    .data(&[TO_NORMAL_ACCOUNT, TO_SOL_TOKEN_ACCOUNT], None)
    .build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(Solana::new(None), http(), get_program_accounts, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux
    .get_program_accounts()
    .data(
      TOKEN_PROGRAM,
      Some(GetProgramAccountsConfig {
        commitment: None,
        data_slice: Some(DataSlice { length: 32, offset: 0 }),
        encoding: Some(AccountEncoding::Base64),
        filters: Some(&[
          Filter::DataSize(165),
          Filter::Memcmp(Memcmp {
            bytes: MemcmpEncodedBytes::Base58(TO_NORMAL_ACCOUNT),
            offset: 32,
          }),
        ]),
        min_context_slot: Some(2),
      }),
    )
    .build();
  assert_eq!(
    trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap().result.unwrap().len(),
    1
  );
});

_create_http_test!(Solana::new(None), http(), get_slot, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.get_slot().data(Some(Commitment::Processed), Some(2)).build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(Solana::new(None), http(), get_slot_leader, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.get_slot_leader().data(Some(Commitment::Processed), Some(2)).build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(Solana::new(None), http(), get_slot_leaders, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.get_slot_leaders().data(1, 2).build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(Solana::new(None), http(), get_token_account_balance, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.get_token_account_balance().data(TO_SOL_TOKEN_ACCOUNT, None).build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(
  Solana::new(None),
  http(),
  get_token_accounts_by_owner,
  |pkgs_aux, trans| async {
    let pkg = &mut pkgs_aux
      .get_token_accounts_by_owner()
      .data(
        TO_NORMAL_ACCOUNT,
        MintOrProgramId::Mint(TO_SOL_TOKEN_MINT),
        Some(GetTokenAccountsByOwnerConfig {
          commitment: None,
          data_slice: None,
          encoding: Some(AccountEncoding::JsonParsed),
          min_context_slot: None,
        }),
      )
      .build();
    let res = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
    let _ = res.result.unwrap().value[0].pubkey;
  }
);

_create_http_test!(Solana::new(None), http(), get_version, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.get_version().build();
  let res = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
  let _ = res.result.unwrap().feature_set;
});

#[cfg(feature = "solana-program")]
_create_http_test!(
  Solana::new(None),
  http(),
  http_get_latest_blockhash_send_transaction_and_get_transaction,
  |pkgs_aux, trans| async {
    let from_keypair = Keypair::from_bytes(&alice_keypair()[..]).unwrap();
    let from_public_key = from_keypair.public.to_bytes();
    let get_latest_blockhash_pkg = &mut pkgs_aux.get_latest_blockhash().data(None, None).build();
    let blockhash = trans
      .send_retrieve_and_decode_contained(get_latest_blockhash_pkg, pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap()
      .value
      .blockhash;
    let message = transfer_message(blockhash, from_public_key);
    let tx = TransactionInput::new(&mut pkgs_aux.byte_buffer, blockhash, message, &[from_keypair])
      .unwrap();
    let send_transaction_pkg = &mut pkgs_aux.send_transaction().data(None, &tx).unwrap().build();
    let tx_hash = trans
      .send_retrieve_and_decode_contained(send_transaction_pkg, pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap();
    assert!(Solana::confirm_transaction(<_>::default(), pkgs_aux, trans, &tx_hash).await.unwrap());

    let get_transaction_pkg0 = &mut pkgs_aux
      .get_transaction()
      .data(
        tx_hash.as_str(),
        Some(GetTransactionConfig {
          commitment: Some(Commitment::Finalized),
          encoding: Some(TransactionEncoding::Base64),
        }),
      )
      .build();
    let _ = trans
      .send_retrieve_and_decode_contained(get_transaction_pkg0, pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap()
      .transaction;

    let get_transaction_pkg1 = &mut pkgs_aux
      .get_transaction()
      .data(
        tx_hash.as_str(),
        Some(GetTransactionConfig {
          commitment: Some(Commitment::Finalized),
          encoding: Some(TransactionEncoding::Json),
        }),
      )
      .build();
    let _ = trans
      .send_retrieve_and_decode_contained(get_transaction_pkg1, pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap()
      .transaction;

    let get_transaction_pkg2 = &mut pkgs_aux
      .get_transaction()
      .data(
        tx_hash.as_str(),
        Some(GetTransactionConfig {
          commitment: Some(Commitment::Finalized),
          encoding: Some(TransactionEncoding::JsonParsed),
        }),
      )
      .build();
    let tx = trans
      .send_retrieve_and_decode_contained(get_transaction_pkg2, pkgs_aux)
      .await
      .unwrap()
      .result
      .unwrap()
      .transaction;
    assert!(matches!(
      generic_tx_parsed_instruction(&tx, 0).unwrap(),
      InstructionJsonParsedInfo::TransferInstruction(..)
    ));
  }
);

_create_http_test!(Solana::new(None), http(), http_reqs_with_array, |pkgs_aux, trans| async {
  let first = &mut pkgs_aux.get_balance().data(TO_NORMAL_ACCOUNT, None).build();
  let second = &mut pkgs_aux.get_balance().data(TO_NORMAL_ACCOUNT, None).build();
  let mut buffer = Vec::new();
  let _ = trans
    .send_retrieve_and_decode_batch(&mut buffer, &mut [first, second][..], pkgs_aux)
    .await
    .unwrap();
});

_create_tokio_tungstenite_test!(
  WS_URL,
  Solana::new(None),
  ws(),
  account_subscribe,
  (account_unsubscribe),
  |pkgs_aux, trans| async {
    let pkg = &mut pkgs_aux
      .account_subscribe()
      .data(TO_NORMAL_ACCOUNT, Some(AccountSubscribeConfig { commitment: None, encoding: None }))
      .build();
    [trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap().result.unwrap()]
  }
);

_create_tokio_tungstenite_test!(
  WS_URL,
  Solana::new(None),
  ws(),
  root_subscribe,
  (root_unsubscribe),
  |pkgs_aux, trans| async {
    let pkg = &mut pkgs_aux.root_subscribe().build();
    [trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap().result.unwrap()]
  }
);

_create_tokio_tungstenite_test!(
  WS_URL,
  Solana::new(None),
  ws(),
  slot_subscribe,
  (slot_unsubscribe),
  |pkgs_aux, trans| async {
    let pkg = &mut pkgs_aux.slot_subscribe().build();
    [trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap().result.unwrap()]
  }
);

_create_tokio_tungstenite_test!(
  WS_URL,
  Solana::new(None),
  ws(),
  slot_updates_subscribe,
  (slots_updates_unsubscribe),
  |pkgs_aux, trans| async {
    let pkg = &mut pkgs_aux.slots_updates_subscribe().build();
    [trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap().result.unwrap()]
  }
);

_create_tokio_tungstenite_test!(
  WS_URL,
  Solana::new(None),
  ws(),
  ws_reqs_with_array,
  (account_unsubscribe, account_unsubscribe),
  |pkgs_aux, trans| async {
    let first = &mut pkgs_aux
      .account_subscribe()
      .data(
        TO_NORMAL_ACCOUNT,
        Some(AccountSubscribeConfig {
          commitment: Some(Commitment::Confirmed),
          encoding: Some(AccountEncoding::JsonParsed),
        }),
      )
      .build();
    let second = &mut pkgs_aux
      .account_subscribe()
      .data(
        TO_NORMAL_ACCOUNT,
        Some(AccountSubscribeConfig {
          commitment: Some(Commitment::Confirmed),
          encoding: Some(AccountEncoding::JsonParsed),
        }),
      )
      .build();
    let mut buffer = Vec::new();
    trans
      .send_retrieve_and_decode_batch(&mut buffer, &mut [first, second], pkgs_aux)
      .await
      .unwrap();
    [*buffer[0].result.as_ref().unwrap(), *buffer[1].result.as_ref().unwrap()]
  }
);

fn alice_keypair() -> [u8; 64] {
  let mut array = [0; 64];
  array[..32].copy_from_slice(&ALICE_SECRET_KEY);
  array[32..].copy_from_slice(&ALICE_PUBLIC_KEY);
  array
}

fn http() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_url("http://localhost:8899").unwrap())
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

fn ws() -> (SerdeJson, WsParams) {
  (SerdeJson, WsParams::default())
}
