use crate::utils::{RequestLimit, _sleep};
use std::time::Instant;

/// Tracks how many requests were performed in a time interval
#[derive(Debug)]
pub struct RequestCounter {
  counter: u64,
  instant: Instant,
}

impl RequestCounter {
  /// If the values defined in [RequestLimit] are in agreement with the **current** values
  /// of [RequestsCounter], then return `T`. Otherwise, awaits until [RequestsCounter] is updated.
  #[inline]
  pub async fn update_params(&mut self, rl: &RequestLimit) {
    if rl.limit() == 0 {
      return;
    }
    let now = Instant::now();
    let duration = *rl.duration();
    let elapsed = now - self.instant;
    if elapsed > duration {
      _debug!("Elapsed is greater than duration. Re-initializing");
      self.counter = 1;
      self.instant = now;
    } else if self.counter == 0 {
      _debug!("First instance call");
      self.counter = 2;
    } else if self.counter == 1 {
      _debug!("First recurrent call");
      if elapsed <= duration {
        let diff = duration - elapsed;
        _debug!("First recurrent call needs to wait {}ms", diff.as_millis());
        _sleep(diff).await;
        self.instant = Instant::now();
      }
      self.counter = self.counter.wrapping_add(1);
    } else if self.counter >= rl.limit() {
      _debug!("Counter exceeded its limit within max duration");
      self.counter = 1;
    } else {
      self.counter = self.counter.wrapping_add(1);
    }
  }
}

impl Default for RequestCounter {
  #[inline]
  fn default() -> Self {
    Self { counter: 0, instant: Instant::now() }
  }
}

#[cfg(feature = "tokio")]
#[cfg(test)]
mod tests {
  use crate::utils::{RequestCounter, RequestLimit};
  use core::time::Duration;
  use std::time::Instant;
  use tokio::time::sleep;

  #[tokio::test]
  #[ignore]
  async fn awaits_when_called_with_counter_reinitialized() {
    const MS: u64 = 1000;
    const MS_DURATION: Duration = Duration::from_millis(MS);

    let rl = RequestLimit::from_ms(2, MS);
    let mut rc = RequestCounter::default();

    async fn test(first_ms: Duration, rc: &mut RequestCounter, rl: &RequestLimit) {
      let first = Instant::now();
      rc.update_params(rl).await;
      assert!(first.elapsed() >= first_ms);

      let second = Instant::now();
      rc.update_params(rl).await;
      assert!(second.elapsed() <= Duration::from_millis(10));
    }

    test(Duration::from_millis(0), &mut rc, &rl).await;
    test(MS_DURATION, &mut rc, &rl).await;
    test(MS_DURATION, &mut rc, &rl).await;
  }

  #[tokio::test]
  async fn counter_is_reinitialized_when_time_expires() {
    let rl = RequestLimit::from_ms(10, 1000);
    let mut rc = RequestCounter::default();
    assert_eq!(rc.counter, 0);
    rc.update_params(&rl).await;
    assert_eq!(rc.counter, 2);
    rc.update_params(&rl).await;
    assert_eq!(rc.counter, 3);
    rc.update_params(&rl).await;
    sleep(Duration::from_millis(1110)).await;
    rc.update_params(&rl).await;
    assert_eq!(rc.counter, 1);
  }

  #[tokio::test]
  async fn does_not_awaits_when_idle_is_greater_than_duration() {
    let rl = RequestLimit::from_ms(2, 50);
    let mut rc = RequestCounter::default();

    async fn test(rc: &mut RequestCounter, rl: &RequestLimit) {
      let now = Instant::now();
      rc.update_params(rl).await;
      assert!(now.elapsed() <= Duration::from_millis(10));
    }

    test(&mut rc, &rl).await;
    sleep(Duration::from_millis(100)).await;
    test(&mut rc, &rl).await;
    sleep(Duration::from_millis(100)).await;
    test(&mut rc, &rl).await;
    sleep(Duration::from_millis(100)).await;
    test(&mut rc, &rl).await;
    sleep(Duration::from_millis(100)).await;
    test(&mut rc, &rl).await;
    sleep(Duration::from_millis(100)).await;
    test(&mut rc, &rl).await;
    sleep(Duration::from_millis(100)).await;
    test(&mut rc, &rl).await;
    sleep(Duration::from_millis(100)).await;
  }

  #[tokio::test]
  async fn has_correct_counter_increment() {
    let rl = RequestLimit::from_ms(2, 100);
    let mut rc = RequestCounter::default();
    assert_eq!(rc.counter, 0);
    rc.update_params(&rl).await;
    assert_eq!(rc.counter, 2);
    rc.update_params(&rl).await;
    assert_eq!(rc.counter, 1);
    rc.update_params(&rl).await;
    assert_eq!(rc.counter, 2);
    rc.update_params(&rl).await;
    assert_eq!(rc.counter, 1);
    rc.update_params(&rl).await;
    assert_eq!(rc.counter, 2);
    rc.update_params(&rl).await;
    assert_eq!(rc.counter, 1);
  }

  #[tokio::test]
  async fn zero_limit_does_not_block_and_does_not_increment() {
    let rl = RequestLimit::from_ms(0, 1_000);
    let mut rc = RequestCounter::default();
    assert_eq!(rc.counter, 0);
    rc.update_params(&rl).await;
    assert_eq!(rc.counter, 0);
    rc.update_params(&rl).await;
    assert_eq!(rc.counter, 0);
    rc.update_params(&rl).await;
    assert_eq!(rc.counter, 0);
    rc.update_params(&rl).await;
    assert_eq!(rc.counter, 0);
    rc.update_params(&rl).await;
    assert_eq!(rc.counter, 0);
    assert!(rc.instant.elapsed() < Duration::from_millis(1000));
  }
}
