//! Ethereum Contract Interface

mod detokenize;
mod options;
mod tokenizable;
mod tokenizable_item;
mod tokenize;

use crate::{
  api::blockchain::ethereum::{
    BlockId, CallRequest, EthCallReq, EthEstimateGasReq, EthGetLogsReq, EthSendTransactionReq,
    Ethereum, FilterBuilder, Log, TransactionRequest,
  },
  network::Transport,
  protocol::{JsonRpcRequest, JsonRpcResponse, ProcessedJsonRpcResponse},
  Pair, Request,
};
use alloc::vec::Vec;
use detokenize::*;
use ethabi::Address;
use ethereum_types::{H256, U256};
use options::*;
use tokenizable::*;
use tokenizable_item::*;
use tokenize::*;

/// Ethereum Contract Interface
#[derive(Debug)]
pub struct Contract<CP, T>
where
  CP: Send,
  T: Transport<Ethereum, CP>,
{
  abi: ethabi::Contract,
  address: Address,
  ethereum: Pair<Ethereum, CP, T>,
}

impl<CP, T> Contract<CP, T>
where
  CP: Send,
  T: Send + Transport<Ethereum, CP>,
{
  /// Creates new Contract Interface given blockchain address and ABI
  #[inline]
  pub fn new(abi: ethabi::Contract, address: Address, eth: Pair<Ethereum, CP, T>) -> Self {
    Contract { address, ethereum: eth, abi }
  }

  /// Creates new Contract Interface given blockchain address and JSON containing ABI
  #[inline]
  pub fn from_json(
    address: Address,
    eth: Pair<Ethereum, CP, T>,
    json: &[u8],
  ) -> ethabi::Result<Self> {
    let abi = ethabi::Contract::load(json)?;
    Ok(Self::new(abi, address, eth))
  }

  /// Get the underlying contract ABI.
  #[inline]
  pub fn abi(&self) -> &ethabi::Contract {
    &self.abi
  }

  /// Returns contract address
  #[inline]
  pub fn address(&self) -> Address {
    self.address
  }

  /// Execute a contract function
  #[inline]
  pub async fn call<FP>(
    &mut self,
    func: &str,
    func_params: FP,
    from: Address,
    options: Options,
  ) -> crate::Result<Option<H256>>
  where
    FP: Tokenize,
    for<'tr> JsonRpcRequest<EthSendTransactionReq<'tr>>: Request<
      CP,
      (),
      ProcessedResponse = ProcessedJsonRpcResponse<Option<H256>>,
      RawResponse = JsonRpcResponse<Option<H256>>,
    >,
  {
    let data = self.abi.function(func)?.encode_input(&func_params.into_tokens())?;
    let Options {
      gas,
      gas_price,
      value,
      nonce,
      condition,
      transaction_type,
      access_list,
      max_fee_per_gas,
      max_priority_fee_per_gas,
    } = options;
    let tr = TransactionRequest {
      from,
      to: Some(self.address),
      gas,
      gas_price,
      value,
      nonce,
      data: Some(crate::api::blockchain::ethereum::Bytes(data)),
      condition,
      transaction_type,
      access_list,
      max_fee_per_gas,
      max_priority_fee_per_gas,
    };
    let (rm, trans) = self.ethereum.parts_mut();
    let req = rm.eth_send_transaction(&tr);
    Ok(trans.send_retrieve_and_decode_one(rm, &req, ()).await?.result)
  }

  /// Estimate gas required for this function call.
  #[inline]
  pub async fn estimate_gas<FP>(
    &mut self,
    func: &str,
    func_params: FP,
    from: Address,
    options: Options,
  ) -> crate::Result<U256>
  where
    FP: Tokenize,
    for<'cr> JsonRpcRequest<EthEstimateGasReq<'cr>>: Request<
      CP,
      (),
      ProcessedResponse = ProcessedJsonRpcResponse<U256>,
      RawResponse = JsonRpcResponse<U256>,
    >,
  {
    let data = self.abi.function(func)?.encode_input(&func_params.into_tokens())?;
    let call_request = CallRequest {
      from: Some(from),
      to: Some(self.address),
      gas: options.gas,
      gas_price: options.gas_price,
      value: options.value,
      data: Some(crate::api::blockchain::ethereum::Bytes(data)),
      transaction_type: options.transaction_type,
      access_list: options.access_list,
      max_fee_per_gas: options.max_fee_per_gas,
      max_priority_fee_per_gas: options.max_priority_fee_per_gas,
    };
    let (rm, trans) = self.ethereum.parts_mut();
    let req = rm.eth_estimate_gas(None, &call_request);
    Ok(trans.send_retrieve_and_decode_one(rm, &req, ()).await?.result)
  }

  /// Find events matching the topics.
  #[inline]
  pub async fn events<AA, BB, CC, R>(
    &mut self,
    event: &str,
    topic0: AA,
    topic1: BB,
    topic2: CC,
  ) -> crate::Result<Vec<R>>
  where
    AA: Tokenize,
    BB: Tokenize,
    CC: Tokenize,
    R: Detokenize,
    for<'filter> JsonRpcRequest<EthGetLogsReq<'filter>>: Request<
      CP,
      (),
      ProcessedResponse = ProcessedJsonRpcResponse<Option<Vec<Log>>>,
      RawResponse = JsonRpcResponse<Option<Vec<Log>>>,
    >,
  {
    fn to_topic<A: Tokenize>(x: A) -> ethabi::Topic<ethabi::Token> {
      let tokens = x.into_tokens();
      if tokens.is_empty() {
        ethabi::Topic::Any
      } else {
        tokens.into()
      }
    }

    let ev = self.abi.event(event)?;

    let topic_filer = ev.filter(ethabi::RawTopicFilter {
      topic0: to_topic(topic0),
      topic1: to_topic(topic1),
      topic2: to_topic(topic2),
    })?;

    let filter = FilterBuilder::default().topic_filter(topic_filer).build();
    let (rm, trans) = self.ethereum.parts_mut();
    let req = rm.eth_get_logs(&filter);
    let logs = if let Some(el) = trans.send_retrieve_and_decode_one(rm, &req, ()).await?.result {
      el
    } else {
      return Ok(Vec::new());
    };

    logs
      .into_iter()
      .map(move |l| {
        let log = ev.parse_log(ethabi::RawLog { topics: l.topics, data: l.data.0 })?;
        R::from_tokens(log.params.into_iter().map(|x| x.value).collect::<Vec<_>>())
      })
      .collect::<crate::Result<Vec<R>>>()
  }

  /// Call constant function
  #[inline]
  pub async fn query<R, A, B, FP>(
    &mut self,
    func: &str,
    func_params: FP,
    from: A,
    options: Options,
    block: B,
  ) -> crate::Result<Option<R>>
  where
    A: Into<Option<Address>>,
    B: Into<Option<BlockId>>,
    FP: Tokenize,
    R: Detokenize,
    for<'cr> JsonRpcRequest<EthCallReq<'cr>>: Request<
      CP,
      (),
      ProcessedResponse = ProcessedJsonRpcResponse<Option<crate::api::blockchain::ethereum::Bytes>>,
      RawResponse = JsonRpcResponse<Option<crate::api::blockchain::ethereum::Bytes>>,
    >,
  {
    let function = self.abi.function(func)?;
    let bytes = function.encode_input(&func_params.into_tokens())?;
    let block_id = block.into();
    let call_request = CallRequest {
      from: from.into(),
      to: Some(self.address),
      gas: options.gas,
      gas_price: options.gas_price,
      value: options.value,
      data: Some(crate::api::blockchain::ethereum::Bytes(bytes)),
      transaction_type: options.transaction_type,
      access_list: options.access_list,
      max_fee_per_gas: options.max_fee_per_gas,
      max_priority_fee_per_gas: options.max_priority_fee_per_gas,
    };
    let (rm, trans) = self.ethereum.parts_mut();
    let req = rm.eth_call(block_id, &call_request);
    trans
      .send_retrieve_and_decode_one(rm, &req, ())
      .await?
      .result
      .map(|el| R::from_tokens(function.decode_output(&el.0)?))
      .transpose()
  }
}

#[cfg(all(feature = "_dev", test))]
mod tests {
  use crate::{
    api::blockchain::ethereum::{
      contract::{Contract, Detokenize, Options},
      BlockId, BlockNumber, CallRequest,
    },
    network::Test,
    protocol::{JsonRpcRequest, JsonRpcResponse},
    Pair,
  };
  use alloc::{
    borrow::{Cow, ToOwned},
    format,
    string::String,
    vec,
    vec::Vec,
  };
  use ethabi::{Address, Token};
  use ethereum_types::{H256, U256};
  use serde::Serialize;

  const HELLO_WORLD: &str =
    "0x00000000000000000000000000000000000000000000000000000000000000200000\
  00000000000000000000000000000000000000000000000000000000000c48656c6c6f20576f726c6421000000000000\
  0000000000000000000000000000";

  #[test]
  fn decoding_array_of_fixed_bytes() {
    let tokens = vec![Token::FixedArray(vec![
      Token::FixedBytes(hex::decode("01").unwrap().into()),
      Token::FixedBytes(hex::decode("02").unwrap().into()),
      Token::FixedBytes(hex::decode("03").unwrap().into()),
      Token::FixedBytes(hex::decode("04").unwrap().into()),
      Token::FixedBytes(hex::decode("05").unwrap().into()),
      Token::FixedBytes(hex::decode("06").unwrap().into()),
      Token::FixedBytes(hex::decode("07").unwrap().into()),
      Token::FixedBytes(hex::decode("08").unwrap().into()),
    ])];
    let data: [[u8; 1]; 8] = Detokenize::from_tokens(tokens).unwrap();
    assert_eq!(data[0][0], 1);
    assert_eq!(data[1][0], 2);
    assert_eq!(data[2][0], 3);
    assert_eq!(data[7][0], 8);
  }

  #[ignore]
  #[test]
  fn decoding_compiles() {
    let _address: Address = output();
    let _bool: bool = output();
    let _bytes: Vec<u8> = output();
    let _string: String = output();
    let _tokens: Vec<Token> = output();
    let _uint: U256 = output();

    let _array: [U256; 4] = output();
    let _bytes: Vec<[[u8; 1]; 64]> = output();
    let _pair: (U256, bool) = output();
    let _vec: Vec<U256> = output();

    let _mixed: (Vec<Vec<u8>>, [U256; 4], Vec<U256>, U256) = output();

    let _uints: (u16, u32, u64, u128) = output();
  }

  #[tokio::test]
  async fn should_call_constant_function() {
    let block_id = BlockId::Number(BlockNumber::Number(1));
    let mut trans = Test::default();
    trans.push_response(response(HELLO_WORLD.into()));
    let result: String = contract(&mut trans)
      .query("name", (), None, Options::default(), block_id)
      .await
      .unwrap()
      .unwrap();
    assert_eq!(result, "Hello World!");
    let mut cr = call_request();
    cr.data = Some(hex::decode("06fdde03").unwrap().into());
    trans.assert_request(&req("eth_call", (cr, block_id)));
    trans.assert_does_not_have_non_asserted_requests();
  }

  #[tokio::test]
  async fn should_call_constant_function_by_hash() {
    let block_id = BlockId::Hash(H256::default());
    let mut trans = Test::default();
    trans.push_response(response(HELLO_WORLD.into()));
    let result: String = contract(&mut trans)
      .query("name", (), None, Options::default(), block_id)
      .await
      .unwrap()
      .unwrap();
    assert_eq!(result, "Hello World!".to_owned());
    let mut cr = call_request();
    cr.data = Some(hex::decode("06fdde03").unwrap().into());
    trans.assert_request(&req("eth_call", (cr, block_id)));
    trans.assert_does_not_have_non_asserted_requests();
  }

  #[tokio::test]
  async fn should_query_with_params() {
    let block_id = BlockId::Number(BlockNumber::Latest);
    let from = Address::from_low_u64_be(5);
    let mut trans = Test::default();
    trans.push_response(response(HELLO_WORLD.into()));
    let result: String = contract(&mut trans)
      .query(
        "name",
        (),
        from,
        Options::with(|options| options.gas_price = Some(10_000_000.into())),
        block_id,
      )
      .await
      .unwrap()
      .unwrap();
    assert_eq!(result, "Hello World!".to_owned());
    let mut cr = call_request();
    cr.data = Some(hex::decode("06fdde03").unwrap().into());
    cr.from = Some(from);
    cr.gas_price = Some(10_000_000.into());
    trans.assert_request(&req("eth_call", (cr, block_id)));
    trans.assert_does_not_have_non_asserted_requests();
  }

  #[tokio::test]
  async fn should_call_a_contract_function() {
    let from = Address::from_low_u64_be(5);
    let mut trans = Test::default();
    trans.push_response(response(format!("{:#x}", H256::from_low_u64_be(5)).into()));
    let result =
      contract(&mut trans).call("name", (), from, Options::default()).await.unwrap().unwrap();
    assert_eq!(result, H256::from_low_u64_be(5));
    let mut cr = call_request();
    cr.data = Some(hex::decode("06fdde03").unwrap().into());
    cr.from = Some(from);
    trans.assert_request(&req("eth_sendTransaction", [cr]));
    trans.assert_does_not_have_non_asserted_requests();
  }

  #[tokio::test]
  async fn should_estimate_gas_usage() {
    let from = Address::from_low_u64_be(5);
    let mut trans = Test::default();
    trans.push_response(response(format!("{:#x}", U256::from(5)).into()));
    let result =
      contract(&mut trans).estimate_gas("name", (), from, Options::default()).await.unwrap();
    assert_eq!(result, 5.into());
    let mut cr = call_request();
    cr.data = Some(hex::decode("06fdde03").unwrap().into());
    cr.from = Some(from);
    trans.assert_request(&req("eth_estimateGas", [cr]));
    trans.assert_does_not_have_non_asserted_requests();
  }

  #[tokio::test]
  async fn should_query_single_parameter_function() {
    let mut trans = Test::default();
    trans.push_response(response(
      "0x0000000000000000000000000000000000000000000000000000000000000020".into(),
    ));
    let result: U256 = contract(&mut trans)
      .query(
        "balanceOf",
        Address::from_low_u64_be(5),
        None,
        Options::default(),
        BlockId::Number(BlockNumber::Latest),
      )
      .await
      .unwrap()
      .unwrap();
    assert_eq!(result, 0x20.into());
    let mut cr = call_request();
    cr.data = Some(
      hex::decode("70a082310000000000000000000000000000000000000000000000000000000000000005")
        .unwrap()
        .into(),
    );
    trans.assert_request(&req("eth_call", (cr, BlockId::Number(BlockNumber::Latest))));
    trans.assert_does_not_have_non_asserted_requests();
  }

  fn call_request() -> CallRequest {
    let mut cr = CallRequest::default();
    cr.to = Some(Address::from_low_u64_be(1));
    cr
  }

  fn contract(trans: &mut Test) -> Contract<(), &mut Test> {
    let pair = Pair::new(trans, ());
    Contract::from_json(Address::from_low_u64_be(1), pair, include_bytes!("./resources/token.json"))
      .unwrap()
  }

  fn output<R>() -> R
  where
    R: Detokenize,
  {
    unimplemented!()
  }

  fn req<P>(method: &'static str, params: P) -> String
  where
    P: Serialize,
  {
    serde_json::to_string(&JsonRpcRequest { id: 1, method, params }).unwrap()
  }

  fn response(result: Cow<'static, str>) -> Cow<'static, str> {
    let elem = JsonRpcResponse { id: 1, method: None, result: Ok(result) };
    serde_json::to_string(&elem).unwrap().into()
  }
}
