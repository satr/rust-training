use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BackendAttempt {
    Success { delay: Duration, value: usize },
    TemporaryFailure { delay: Duration },
    PermanentFailure { delay: Duration },
}

#[derive(Debug, PartialEq, Eq)]
enum RequestError {
    TemporaryFailure,
    PermanentFailure,
    TimedOut,
    RetryLimitExceeded,
}

#[allow(dead_code)]
async fn call_backend(attempt: BackendAttempt) -> Result<usize, RequestError> {
    match attempt {
        BackendAttempt::Success { delay, value } => {
            tokio::time::sleep(delay).await;
            Ok(value)
        }
        BackendAttempt::TemporaryFailure { delay } => {
            tokio::time::sleep(delay).await;
            Err(RequestError::TemporaryFailure)
        }
        BackendAttempt::PermanentFailure { delay } => {
            tokio::time::sleep(delay).await;
            Err(RequestError::PermanentFailure)
        }
    }
}

async fn request_with_timeout(
    attempt: BackendAttempt,
    per_attempt_timeout: Duration,
) -> Result<usize, RequestError> {
    Ok(
        tokio::time::timeout(per_attempt_timeout, call_backend(attempt))
            .await
            .map_err(|_| RequestError::TimedOut)??,
    )
}

async fn request_with_retries(
    attempts: Vec<BackendAttempt>,
    per_attempt_timeout: Duration,
    max_attempts: usize,
) -> Result<usize, RequestError> {
    if max_attempts == 0 {
        return Err(RequestError::RetryLimitExceeded);
    }

    for (attempt_number, attempt) in attempts.into_iter().enumerate() {
        if attempt_number == max_attempts {
            break;
        }

        match request_with_timeout(attempt, per_attempt_timeout).await {
            Ok(value) => return Ok(value),
            Err(RequestError::PermanentFailure) => {
                return Err(RequestError::PermanentFailure);
            }
            Err(RequestError::TemporaryFailure | RequestError::TimedOut)
                if attempt_number + 1 == max_attempts =>
            {
                return Err(RequestError::RetryLimitExceeded);
            }
            Err(RequestError::TemporaryFailure | RequestError::TimedOut) => {}
            Err(RequestError::RetryLimitExceeded) => {
                return Err(RequestError::RetryLimitExceeded);
            }
        }
    }

    Err(RequestError::RetryLimitExceeded)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Tokio timeout and retry exercise");
}

#[cfg(test)]
mod tests {
    use super::{request_with_retries, request_with_timeout, BackendAttempt, RequestError};
    use std::time::Duration;

    fn ms(value: u64) -> Duration {
        Duration::from_millis(value)
    }

    #[tokio::test(flavor = "current_thread")]
    async fn successful_request_finishes_before_timeout() {
        let value = request_with_timeout(
            BackendAttempt::Success {
                delay: ms(1),
                value: 42,
            },
            ms(50),
        )
        .await
        .expect("fast request should succeed");

        assert_eq!(value, 42);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn slow_request_returns_timeout() {
        let error = request_with_timeout(
            BackendAttempt::Success {
                delay: ms(50),
                value: 42,
            },
            ms(1),
        )
        .await
        .expect_err("slow request should time out");

        assert_eq!(error, RequestError::TimedOut);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn retries_temporary_failure_then_succeeds() {
        let value = request_with_retries(
            vec![
                BackendAttempt::TemporaryFailure { delay: ms(1) },
                BackendAttempt::Success {
                    delay: ms(1),
                    value: 7,
                },
            ],
            ms(50),
            3,
        )
        .await
        .expect("second attempt should succeed");

        assert_eq!(value, 7);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn permanent_failure_is_not_retried() {
        let error = request_with_retries(
            vec![
                BackendAttempt::PermanentFailure { delay: ms(1) },
                BackendAttempt::Success {
                    delay: ms(1),
                    value: 7,
                },
            ],
            ms(50),
            2,
        )
        .await
        .expect_err("permanent failure should stop retries");

        assert_eq!(error, RequestError::PermanentFailure);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn retry_limit_is_reported_after_retryable_failures() {
        let error = request_with_retries(
            vec![
                BackendAttempt::TemporaryFailure { delay: ms(1) },
                BackendAttempt::Success {
                    delay: ms(50),
                    value: 7,
                },
            ],
            ms(1),
            2,
        )
        .await
        .expect_err("temporary failure plus timeout should exhaust attempts");

        assert_eq!(error, RequestError::RetryLimitExceeded);
    }
}
