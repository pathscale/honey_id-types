
# API Reference

## Structs/Datamodels

```rust

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
|111|CreateAppConfig|`appPublicId: Nanoid<16, Base62Alphabet>`, `callBackUrl: String`|`appPublicId: Nanoid<16, Base62Alphabet>`, `createdAt: i64`, `appApiKey: String`, `minPasswordLength: i32`, `requiredPasswordChars: String`|Platform can create new apps|false|InternalError(ErrorCode::InternalError)|
|112|BanUser|`userPublicId: Nanoid<16, Base62Alphabet>`, `appPublicId: Nanoid<16, Base62Alphabet>`||Ban a user from provided app|false|UserNotFound(ErrorCode::NotFound), AppNotFound(ErrorCode::NotFound), MembershipNotFound(ErrorCode::NotFound)|
|113|UnbanUser|`userPublicId: Nanoid<16, Base62Alphabet>`, `appPublicId: Nanoid<16, Base62Alphabet>`||Unban a user from a specific app|false|UserNotFound(ErrorCode::NotFound), AppNotFound(ErrorCode::NotFound), MembershipNotFound(ErrorCode::NotFound)|
|114|DeleteUser|`appPublicId: Nanoid<16, Base62Alphabet>`, `userPublicId: Nanoid<16, Base62Alphabet>`||Delete a user|false|InternalError(ErrorCode::InternalError)|
|115|DeleteAppConfig|`appPublicId: Nanoid<16, Base62Alphabet>`||Delete app configuration|false|InternalError(ErrorCode::InternalError)|
|116|EditAppConfig|`appPublicId: Nanoid<16, Base62Alphabet>`, `callBackUrl: Option<String>`, `minPasswordLength: Option<i32>`, `requiredPasswordChars: Option<String>`|`appPublicId: Nanoid<16, Base62Alphabet>`, `callBackUrl: String`, `minPasswordLength: i32`, `requiredPasswordChars: String`|Edit app configuration|false|AppNotFound(ErrorCode::NotFound), InternalError(ErrorCode::InternalError)|
|117|GetAppSecurityRules|`appPublicId: Nanoid<16, Base62Alphabet>`|`appPublicId: Nanoid<16, Base62Alphabet>`, `minPasswordLength: i32`, `requiredPasswordChars: String`|Get security rules contained within current app's configuration|false|AppNotFound(ErrorCode::NotFound)|
|118|SetLogLevel|`logLevel: Option<LogLevel>`|`logLevel: LogLevel`|Set log level at runtime|false|InvalidLogLevel(ErrorCode::BadRequest)|

## authEndpoints Server
ID: 20
### Endpoints
|Code|Name|Parameters|Response|Description|FE Facing|Errors|
|-----------|-----------|----------|--------|-----------|-----------|-----------|
|200|ApiKeyConnect|`appApiKey: String`|||false|InvalidApiKey(ErrorCode::Unauthorized)|
|201|AuthorizedConnect|`accessToken: String`|||true|InvalidAccessToken(ErrorCode::Unauthorized)|

## beCallbackApi Server
ID: 21
### Endpoints
|Code|Name|Parameters|Response|Description|FE Facing|Errors|
|-----------|-----------|----------|--------|-----------|-----------|-----------|
|210|ReceiveToken|`token: String`, `username: String`, `userPubId: Nanoid<16, Base62Alphabet>`||Backend receives auth tokens, happens after login|false|InvalidToken(ErrorCode::BadRequest)|
|211|ReceiveUserInfo|`userPubId: Nanoid<16, Base62Alphabet>`, `username: String`, `appPubId: Option<Nanoid<16, Base62Alphabet>>`, `token: Option<String>`||Backend receives user info with optional token, happens after new user signs up. Platform app also receives this so that it can maintain records of app users, in which case Token will be set to None|false|InvalidToken(ErrorCode::BadRequest)|
|212|ReceiveUserDeleted|`userPubId: Nanoid<16, Base62Alphabet>`, `appPubId: Option<Nanoid<16, Base62Alphabet>>`||Backend receives notification when a user is deleted or banned. App should clean up all user data and invalidate tokens.|false||
|213|ValidateToken|`token: String`|`valid: bool`, `userPubId: Option<Nanoid<16, Base62Alphabet>>`|App validates an existing token and returns whether it is valid along with the associated userPubId|false|InvalidToken(ErrorCode::BadRequest)|
