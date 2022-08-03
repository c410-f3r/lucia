use crate::blockchain::ethereum::{BlockNumber, Filter, ValueOrArray};
use alloc::{vec, vec::Vec};
use ethereum_types::{H160, H256};

/// Filter Builder
#[derive(Debug, Default)]
pub struct FilterBuilder {
  filter: Filter,
}

impl FilterBuilder {
  /// Sets `from_block`. The fields `from_block` and `block_hash` are
  /// mutually exclusive. Setting `from_block` will clear a previously set
  /// `block_hash`.
  #[inline]
  pub fn from_block(mut self, block: BlockNumber) -> Self {
    self.filter.block_hash = None;
    self.filter.from_block = Some(block);
    self
  }

  /// Sets `to_block`. The fields `to_block` and `block_hash` are mutually
  /// exclusive. Setting `to_block` will clear a previously set `block_hash`.
  #[inline]
  pub fn to_block(mut self, block: BlockNumber) -> Self {
    self.filter.block_hash = None;
    self.filter.to_block = Some(block);
    self
  }

  /// Sets `block_hash`. The field `block_hash` and the pair `from_block` and
  /// `to_block` are mutually exclusive. Setting `block_hash` will clear a
  /// previously set `from_block` and `to_block`.
  #[inline]
  pub fn block_hash(mut self, hash: H256) -> Self {
    self.filter.from_block = None;
    self.filter.to_block = None;
    self.filter.block_hash = Some(hash);
    self
  }

  /// Single address
  #[inline]
  pub fn address(mut self, address: Vec<H160>) -> Self {
    self.filter.address = Some(ValueOrArray(address));
    self
  }

  /// Topics
  #[inline]
  pub fn topics(
    mut self,
    topic1: Option<Vec<H256>>,
    topic2: Option<Vec<H256>>,
    topic3: Option<Vec<H256>>,
    topic4: Option<Vec<H256>>,
  ) -> Self {
    let mut topics = vec![topic1, topic2, topic3, topic4]
      .into_iter()
      .rev()
      .skip_while(Option::is_none)
      .map(|option| option.map(ValueOrArray))
      .collect::<Vec<_>>();
    topics.reverse();

    self.filter.topics = Some(topics);
    self
  }

  /// Sets the topics according to the given `ethabi` topic filter
  #[inline]
  pub fn topic_filter(self, topic_filter: ethabi::TopicFilter) -> Self {
    self.topics(
      topic_to_option(topic_filter.topic0),
      topic_to_option(topic_filter.topic1),
      topic_to_option(topic_filter.topic2),
      topic_to_option(topic_filter.topic3),
    )
  }

  /// Limit the result
  #[inline]
  pub fn limit(mut self, limit: usize) -> Self {
    self.filter.limit = Some(limit);
    self
  }

  /// Returns filter
  #[inline]
  pub fn build(self) -> Filter {
    self.filter
  }
}

/// Converts a `Topic` to an equivalent `Option<Vec<T>>`, suitable for `FilterBuilder::topics`
fn topic_to_option<T>(topic: ethabi::Topic<T>) -> Option<Vec<T>> {
  match topic {
    ethabi::Topic::Any => None,
    ethabi::Topic::OneOf(v) => Some(v),
    ethabi::Topic::This(t) => Some(vec![t]),
  }
}
