
# API Reference

## Structs/Datamodels

```rust
struct AppKeyInfo{ keyId: String, label: String, createdAt: i64, lastUsedAt: Option<i64>, expiresAt: Option<i64>, revokedAt: Option<i64> }

```
---

## Enums

```rust
enum LogLevel { off, error, warn, info, debug, trace, detail }


enum UserRole { Public, PlatformAdmin, PlatformSupport, AppNewUser, AppAdmin, AppSupport, AppApiKey, Platform }


enum UserStatus { enabled, disabled, banned }


enum ErrorCode { BadRequest, Unauthorized, PaymentRequired, Forbidden, NotFound, MethodNotAllowed, NotAcceptable, ProxyAuthenticationRequired, RequestTimeout, Conflict, Gone, LengthRequired, PreconditionFailed, PayloadTooLarge, UriTooLong, UnsupportedMediaType, RangeNotSatisfiable, ExpectationFailed, ImATeapot, MisdirectedRequest, UnprocessableEntity, Locked, FailedDependency, UpgradeRequired, PreconditionRequired, TooManyRequests, RequestHeaderFieldsTooLarge, UnavailableForLegalReasons, InternalError, NotImplemented, BadGateway, ServiceUnavailable, GatewayTimeout, HttpVersionNotSupported, VariantAlsoNegotiates, InsufficientStorage, LoopDetected, NotExtended, NetworkAuthenticationRequired }

```
---

        

## publicApiConnection Server
ID: 0
### Endpoints
|Code|Name|Parameters|Response|Description|FE Facing|Errors|
|-----------|-----------|----------|--------|-----------|-----------|-----------|
|0|PublicConnect|||Initiates a websocket connection session with that permits access to endpoints with the UserRole::Public role|true||

## publicAuthApi Server
ID: 1
### Endpoints
|Code|Name|Parameters|Response|Description|FE Facing|Errors|
|-----------|-----------|----------|--------|-----------|-----------|-----------|
|10|Signup|`appPublicId: Nanoid<16, Base62Alphabet>`, `username: String`, `password: String`|`accessToken: String`, `encryptionKey: String`|Frontend creates new user account.|true|InvalidUsername(ErrorCode::BadRequest), AppNotFound(ErrorCode::NotFound), AccountForbidden(ErrorCode::Forbidden), InvalidPassword(ErrorCode::Unauthorized), CallbackFailed(ErrorCode::BadGateway)|
|12|SubmitUsername|`appPublicId: Nanoid<16, Base62Alphabet>`, `username: String`|`expiresAt: i64`|Step 1: Frontend submits username during auth flow.|true|AppNotFound(ErrorCode::NotFound), UserNotFound(ErrorCode::NotFound), AccountForbidden(ErrorCode::Forbidden)|
|13|SubmitPassword|`password: String`|`accessToken: String`, `encryptionKey: String`|Step 2: Frontend submits password to complete HoneyAuth login. Session is per connection. Returns tokens and token metadata.|true|AuthFlowRequired(ErrorCode::BadRequest), InvalidPassword(ErrorCode::Unauthorized), AccessDenied(ErrorCode::Forbidden), CallbackFailed(ErrorCode::BadGateway)|

## platformApiKeyConnection Server
ID: 10
### Endpoints
|Code|Name|Parameters|Response|Description|FE Facing|Errors|
|-----------|-----------|----------|--------|-----------|-----------|-----------|
|100|PlatformConnect|`platformApiKey: String`||Handles platform API Key login to initiate the connection session between Honey API Backend and this server|false|InvalidApiKey(ErrorCode::Unauthorized)|

## platformApi Server
ID: 11
### Endpoints
|Code|Name|Parameters|Response|Description|FE Facing|Errors|
|-----------|-----------|----------|--------|-----------|-----------|-----------|
|111|CreateAppConfig|`appPublicId: Nanoid<16, Base62Alphabet>`, `callBackUrl: String`|`appPublicId: Nanoid<16, Base62Alphabet>`, `createdAt: i64`, `appApiKey: String`, `callbackApiKey: String`, `minPasswordLength: i32`, `requiredPasswordChars: String`|Platform creates an app and returns its app key and separate callback credential once|false|InternalError(ErrorCode::InternalError)|
|112|BanUser|`userPublicId: Nanoid<16, Base62Alphabet>`, `appPublicId: Nanoid<16, Base62Alphabet>`||Ban a user from provided app|false|UserNotFound(ErrorCode::NotFound), AppNotFound(ErrorCode::NotFound), MembershipNotFound(ErrorCode::NotFound)|
|113|UnbanUser|`userPublicId: Nanoid<16, Base62Alphabet>`, `appPublicId: Nanoid<16, Base62Alphabet>`||Unban a user from a specific app|false|UserNotFound(ErrorCode::NotFound), AppNotFound(ErrorCode::NotFound), MembershipNotFound(ErrorCode::NotFound)|
|114|DeleteUser|`appPublicId: Nanoid<16, Base62Alphabet>`, `userPublicId: Nanoid<16, Base62Alphabet>`||Delete a user|false|InternalError(ErrorCode::InternalError)|
|115|DeleteAppConfig|`appPublicId: Nanoid<16, Base62Alphabet>`||Delete app configuration|false|InternalError(ErrorCode::InternalError)|
|116|EditAppConfig|`appPublicId: Nanoid<16, Base62Alphabet>`, `callBackUrl: Option<String>`, `minPasswordLength: Option<i32>`, `requiredPasswordChars: Option<String>`|`appPublicId: Nanoid<16, Base62Alphabet>`, `callBackUrl: String`, `minPasswordLength: i32`, `requiredPasswordChars: String`|Edit app configuration|false|AppNotFound(ErrorCode::NotFound), InternalError(ErrorCode::InternalError)|
|117|GetAppSecurityRules|`appPublicId: Nanoid<16, Base62Alphabet>`|`appPublicId: Nanoid<16, Base62Alphabet>`, `minPasswordLength: i32`, `requiredPasswordChars: String`|Get security rules contained within current app's configuration|false|AppNotFound(ErrorCode::NotFound)|
|118|SetLogLevel|`logLevel: Option<LogLevel>`|`logLevel: LogLevel`|Set log level at runtime|false|InvalidLogLevel(ErrorCode::BadRequest)|
|119|RegenerateAppApiKey|`appPublicId: Nanoid<16, Base62Alphabet>`|`appApiKey: String`|Compatibility rotation: creates a new app key and immediately revokes every other app key|false|AppNotFound(ErrorCode::NotFound), InternalError(ErrorCode::InternalError)|
|120|GetUserSecurity|`userPublicId: Nanoid<16, Base62Alphabet>`|`totpEnabled: bool`, `telegramUsername: Option<String>`, `telegramConfirmed: bool`|A user's second factors: whether an authenticator app is enrolled, and the Telegram account bound for recovery, if any|false|UserNotFound(ErrorCode::NotFound)|
|121|CreateAppKey|`appPublicId: Nanoid<16, Base62Alphabet>`, `label: String`, `expiresInSecs: Option<i64>`, `createdBy: String`|`keyId: String`, `key: String`, `label: String`|Creates a hashed app key and returns its key string once|false|AppNotFound(ErrorCode::NotFound), InvalidKeyRequest(ErrorCode::BadRequest), InternalError(ErrorCode::InternalError)|
|122|ListAppKeys|`appPublicId: Nanoid<16, Base62Alphabet>`|`keys: Vec<AppKeyInfo>`|Lists app key metadata without secrets or hashes|false|AppNotFound(ErrorCode::NotFound)|
|123|RevokeAppKey|`appPublicId: Nanoid<16, Base62Alphabet>`, `keyId: String`, `graceSecs: i64`|`keyId: String`, `expiresAt: Option<i64>`, `revokedAt: Option<i64>`|Revokes an app key now or schedules its expiry after a grace period|false|AppNotFound(ErrorCode::NotFound), InvalidGracePeriod(ErrorCode::BadRequest), InternalError(ErrorCode::InternalError)|
|124|RotateAppCallbackCredential|`appPublicId: Nanoid<16, Base62Alphabet>`|`appPublicId: Nanoid<16, Base62Alphabet>`, `callbackApiKey: String`|Creates and returns a separate callback credential once|false|AppNotFound(ErrorCode::NotFound), InternalError(ErrorCode::InternalError)|
|125|InspectAppCredential|`appPublicId: Nanoid<16, Base62Alphabet>`, `appKey: String`|`appPublicId: Nanoid<16, Base62Alphabet>`, `keyId: String`, `service: bool`, `expiresAt: Option<i64>`, `revokedAt: Option<i64>`|Proves an app credential and returns its key status metadata to the trusted API backend|false|AppNotService(ErrorCode::Forbidden), InvalidCredential(ErrorCode::Unauthorized), InternalError(ErrorCode::InternalError)|
|126|SetAppService|`appPublicId: Nanoid<16, Base62Alphabet>`, `service: bool`|`service: bool`|Sets whether an app may act as a service verifier client|false|AppNotFound(ErrorCode::NotFound), InternalError(ErrorCode::InternalError)|

## authEndpoints Server
ID: 20
### Endpoints
|Code|Name|Parameters|Response|Description|FE Facing|Errors|
|-----------|-----------|----------|--------|-----------|-----------|-----------|
|200|ApiKeyConnect|`appApiKey: String`||Authenticates Honey Auth callbacks using the app's separate callback credential|false|InvalidApiKey(ErrorCode::Unauthorized)|
|201|AuthorizedConnect|`accessToken: String`||A user connects to an app with the access token auth issued them.|true|InvalidAccessToken(ErrorCode::Unauthorized)|

## beCallbackApi Server
ID: 21
### Endpoints
|Code|Name|Parameters|Response|Description|FE Facing|Errors|
|-----------|-----------|----------|--------|-----------|-----------|-----------|
|210|ReceiveToken|`token: String`, `username: String`, `userPubId: Nanoid<16, Base62Alphabet>`, `completionId: Option<String>`|`completionId: Option<String>`|Backend receives auth tokens after login. completionId opts into a revocation-aware completion adapter; legacy handlers reject it. Retries of pending work resume safely, completed duplicates do not reapply effects, and deleted users cannot be restored by callback replay. Requests without completionId retain legacy behavior.|false|InvalidToken(ErrorCode::BadRequest), CompletionNotSupported(ErrorCode::NotImplemented), InvalidCompletionId(ErrorCode::BadRequest), CompletionConflict(ErrorCode::Conflict), CompletionRevoked(ErrorCode::Conflict)|
|211|ReceiveUserInfo|`userPubId: Nanoid<16, Base62Alphabet>`, `username: String`, `appPubId: Option<Nanoid<16, Base62Alphabet>>`, `token: Option<String>`||Backend receives user info with optional token, happens after new user signs up. Platform app also receives this so that it can maintain records of app users, in which case Token will be set to None|false|InvalidToken(ErrorCode::BadRequest)|
|212|ReceiveUserDeleted|`userPubId: Nanoid<16, Base62Alphabet>`, `appPubId: Option<Nanoid<16, Base62Alphabet>>`||Backend receives notification when a user is deleted or banned. App should clean up all user data and invalidate tokens.|false||
|213|ValidateToken|`token: String`|`valid: bool`, `userPubId: Option<Nanoid<16, Base62Alphabet>>`|App validates an existing token and returns whether it is valid along with the associated userPubId|false|InvalidToken(ErrorCode::BadRequest)|
