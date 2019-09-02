//! The APNs response types

use std::fmt;

/// The response data from APNs.
#[derive(Debug)]
pub struct Response {
    /// If the notification was not successful, has the body content from APNs.
    pub error: Option<ErrorBody>,

    /// Is the value defined in the `NotificationOptions` or a new Uuid
    /// generated by APNs.
    pub apns_id: Option<String>,

    /// The HTTP response code.
    ///
    /// * 200 Success
    /// * 400 Bad request
    /// * 403 There was an error with the certificate or with the provider authentication token
    /// * 405 The request used a bad `:method` value. Only `POST` requests are supported.
    /// * 410 The device token is no longer active for the topic.
    /// * 413 The notification payload was too large.
    /// * 429 The server received too many requests for the same device token.
    /// * 500 Internal server error.
    /// * 503 The server is shutting down and unavailable.
    pub code: u16,
}

/// The response body from APNs. Only available for errors.
#[derive(Deserialize, Debug, PartialEq)]
pub struct ErrorBody {
    /// The error indicating the reason for the failure.
    pub reason: ErrorReason,

    /// If the value of the `ErrorReason` is `Unregistered`, the value of this
    /// key is the last time at which APNs confirmed that the device token was
    /// no longer valid for the topic.
    ///
    /// Stop pushing notifications until the device registers a token with a
    /// later timestamp with your provider.
    pub timestamp: Option<u64>,
}

/// A description what went wrong with the push notification.
#[derive(Deserialize, Debug, PartialEq)]
pub enum ErrorReason {
    /// The collapse identifier exceeds the maximum allowed size.
    BadCollapseId,

    /// The specified device token was bad. Verify that the request contains a
    /// valid token and that the token matches the environment.
    BadDeviceToken,

    /// The `apns_expiration` in `NotificationOptions` is bad.
    BadExpirationDate,

    /// The `apns_id` in `NotificationOptions` is bad.
    BadMessageId,

    /// The `apns_priority` in `NotificationOptions` is bad.
    BadPriority,

    /// The `apns_topic` in `NotificationOptions` is bad.
    BadTopic,

    /// The device token does not match the specified topic.
    DeviceTokenNotForTopic,

    /// One or more headers were repeated.
    DuplicateHeaders,

    /// Idle time out.
    IdleTimeout,

    /// The device token is not specified in the payload.
    MissingDeviceToken,

    /// The `apns_topic` of the `NotificationOptions` was not specified and was required.
    /// The `apns_topic` header is mandatory when the client is connected using the
    /// `CertificateConnector` and the included PKCS12 file includes multiple topics,
    /// or when using the `TokenConnector`.
    MissingTopic,

    /// The message payload was empty.
    PayloadEmpty,

    /// Pushing to this topic is not allowed.
    TopicDisallowed,

    /// The certificate was bad.
    BadCertificate,

    /// The client certificate was for the wrong environment.
    BadCertificateEnvironment,

    /// The provider token is stale and a new token should be generated.
    ExpiredProviderToken,

    /// The specified action is not allowed.
    Forbidden,

    /// The provider token is not valid or the token signature could not be verified.
    InvalidProviderToken,

    /// No provider certificate was used to connect to APNs and Authorization
    /// header was missing or no provider token was specified.
    MissingProviderToken,

    /// The request path value is bad.
    BadPath,

    /// The request method was not `POST`.
    MethodNotAllowed,

    /// The device token is inactive for the specified topic. You should stop sending
    /// notifications to this token.
    Unregistered,

    /// The message payload was too large (4096 bytes)
    PayloadTooLarge,

    /// The provider token is being updated too often.
    TooManyProviderTokenUpdates,

    /// Too many requests were made consecutively to the same device token.
    TooManyRequests,

    /// An internal server error occurred.
    InternalServerError,

    /// The service is unavailable.
    ServiceUnavailable,

    /// The server is shutting down.
    Shutdown,
}

impl fmt::Display for ErrorReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match *self {
            ErrorReason::BadCollapseId =>
                "The collapse identifier exceeds the maximum allowed size.",
            ErrorReason::BadDeviceToken =>
                "The specified device token was bad. Verify that the request contains a valid token and that the token matches the environment.",
            ErrorReason::BadExpirationDate =>
                "The `apns_expiration` in `NotificationOptions` is bad.",
            ErrorReason::BadMessageId =>
                "The `apns_id` in `NotificationOptions` is bad.",
            ErrorReason::BadPriority =>
                "The `apns_priority` in `NotificationOptions` is bad.",
            ErrorReason::BadTopic =>
                "The `apns_topic` in `NotificationOptions` is bad.",
            ErrorReason::DeviceTokenNotForTopic =>
                "The device token does not match the specified topic.",
            ErrorReason::DuplicateHeaders =>
                "One or more headers were repeated.",
            ErrorReason::IdleTimeout =>
                "Idle time out.",
            ErrorReason::MissingDeviceToken =>
                "The device token is not specified in the payload.",
            ErrorReason::MissingTopic =>
                "The `apns_topic` of the `NotificationOptions` was not specified and was required. The `apns_topic` header is mandatory when the client is connected using the `CertificateConnector` and the included PKCS12 file includes multiple topics, or when using the `TokenConnector`.",
            ErrorReason::PayloadEmpty =>
                "The message payload was empty.",
            ErrorReason::TopicDisallowed =>
                "Pushing to this topic is not allowed.",
            ErrorReason::BadCertificate =>
                "The certificate was bad.",
            ErrorReason::BadCertificateEnvironment =>
                "The client certificate was for the wrong environment.",
            ErrorReason::ExpiredProviderToken =>
                "The provider token is stale and a new token should be generated.",
            ErrorReason::Forbidden =>
                "The specified action is not allowed.",
            ErrorReason::InvalidProviderToken =>
                "The provider token is not valid or the token signature could not be verified.",
            ErrorReason::MissingProviderToken =>
                "No provider certificate was used to connect to APNs and Authorization header was missing or no provider token was specified.",
            ErrorReason::BadPath =>
                "The request path value is bad.",
            ErrorReason::MethodNotAllowed =>
                "The request method was not `POST`.",
            ErrorReason::Unregistered =>
                "The device token is inactive for the specified topic. You should stop sending notifications to this token.",
            ErrorReason::PayloadTooLarge =>
                "The message payload was too large (4096 bytes)",
            ErrorReason::TooManyProviderTokenUpdates =>
                "The provider token is being updated too often.",
            ErrorReason::TooManyRequests =>
                "Too many requests were made consecutively to the same device token.",
            ErrorReason::InternalServerError =>
                "An internal server error occurred.",
            ErrorReason::ServiceUnavailable =>
                "The service is unavailable.",
            ErrorReason::Shutdown =>
                "The server is shutting down.",
        };

        f.write_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_error_response_parsing() {
        let errors = vec![
            (ErrorReason::BadCollapseId, "BadCollapseId", None),
            (ErrorReason::BadDeviceToken, "BadDeviceToken", None),
            (ErrorReason::BadExpirationDate, "BadExpirationDate", None),
            (ErrorReason::BadMessageId, "BadMessageId", None),
            (ErrorReason::BadPriority, "BadPriority", None),
            (ErrorReason::BadTopic, "BadTopic", None),
            (
                ErrorReason::DeviceTokenNotForTopic,
                "DeviceTokenNotForTopic",
                None,
            ),
            (ErrorReason::DuplicateHeaders, "DuplicateHeaders", None),
            (ErrorReason::IdleTimeout, "IdleTimeout", None),
            (ErrorReason::MissingDeviceToken, "MissingDeviceToken", None),
            (ErrorReason::MissingTopic, "MissingTopic", None),
            (ErrorReason::PayloadEmpty, "PayloadEmpty", None),
            (ErrorReason::TopicDisallowed, "TopicDisallowed", None),
            (ErrorReason::BadCertificate, "BadCertificate", None),
            (
                ErrorReason::BadCertificateEnvironment,
                "BadCertificateEnvironment",
                None,
            ),
            (
                ErrorReason::ExpiredProviderToken,
                "ExpiredProviderToken",
                None,
            ),
            (ErrorReason::Forbidden, "Forbidden", None),
            (
                ErrorReason::InvalidProviderToken,
                "InvalidProviderToken",
                None,
            ),
            (
                ErrorReason::MissingProviderToken,
                "MissingProviderToken",
                None,
            ),
            (ErrorReason::BadPath, "BadPath", None),
            (ErrorReason::MethodNotAllowed, "MethodNotAllowed", None),
            (
                ErrorReason::Unregistered,
                "Unregistered",
                Some(1508249865488u64),
            ),
            (ErrorReason::PayloadTooLarge, "PayloadTooLarge", None),
            (
                ErrorReason::TooManyProviderTokenUpdates,
                "TooManyProviderTokenUpdates",
                None,
            ),
            (ErrorReason::TooManyRequests, "TooManyRequests", None),
            (
                ErrorReason::InternalServerError,
                "InternalServerError",
                None,
            ),
            (ErrorReason::ServiceUnavailable, "ServiceUnavailable", None),
            (ErrorReason::Shutdown, "Shutdown", None),
        ];

        for error in errors.into_iter() {
            let response_data = match error.2 {
                None => json!({"reason": error.1}),
                Some(ts) => json!({"reason": error.1, "timestamp": ts}),
            };

            let response_string = serde_json::to_string(&response_data).unwrap();

            let response_body: ErrorBody = serde_json::from_str(&response_string).unwrap();

            let expected_body = match error.2 {
                None => ErrorBody {
                    reason: error.0,
                    timestamp: None,
                },
                Some(ts) => ErrorBody {
                    reason: error.0,
                    timestamp: Some(ts),
                },
            };

            assert_eq!(expected_body, response_body);
        }
    }
}