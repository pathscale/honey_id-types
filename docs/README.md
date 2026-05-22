
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

```
---

        

## publicApiConnection Server
ID: 0
### Endpoints
|Code|Name|Parameters|Response|Description|FE Facing|
|-----------|-----------|----------|--------|-----------|-----------|
|0|PublicConnect|||Initiates a websocket connection session with that permits access to endpoints with the UserRole::Public role|true|

## publicAuthApi Server
ID: 1
### Endpoints
|Code|Name|Parameters|Response|Description|FE Facing|
|-----------|-----------|----------|--------|-----------|-----------|
|10|Signup|`appPublicId: Nanoid<16, Base62Alphabet>`, `username: String`, `password: String`|`accessToken: String`|Frontend creates new user account.|true|
|12|SubmitUsername|`appPublicId: Nanoid<16, Base62Alphabet>`, `username: String`|`expiresAt: i64`|Step 1: Frontend submits username during auth flow.|true|
|13|SubmitPassword|`password: String`|`accessToken: String`|Step 2: Frontend submits password to complete HoneyAuth login. Session is per connection. Returns tokens and token metadata.|true|

## platformApiKeyConnection Server
ID: 10
### Endpoints
|Code|Name|Parameters|Response|Description|FE Facing|
|-----------|-----------|----------|--------|-----------|-----------|
|100|PlatformConnect|`platformApiKey: String`||Handles platform API Key login to initiate the connection session between Honey API Backend and this server|false|

## platformApi Server
ID: 11
### Endpoints
|Code|Name|Parameters|Response|Description|FE Facing|
|-----------|-----------|----------|--------|-----------|-----------|
|111|CreateAppConfig|`appPublicId: Nanoid<16, Base62Alphabet>`, `callBackUrl: String`|`appPublicId: Nanoid<16, Base62Alphabet>`, `createdAt: i64`, `appApiKey: String`, `minPasswordLength: i32`, `requiredPasswordChars: String`|Platform can create new apps|false|
|112|BanUser|`userPublicId: Nanoid<16, Base62Alphabet>`, `appPublicId: Nanoid<16, Base62Alphabet>`||Ban a user from provided app|false|
|113|UnbanUser|`userPublicId: Nanoid<16, Base62Alphabet>`, `appPublicId: Nanoid<16, Base62Alphabet>`||Unban a user from a specific app|false|
|114|DeleteUser|`appPublicId: Nanoid<16, Base62Alphabet>`, `userPublicId: Nanoid<16, Base62Alphabet>`||Delete a user|false|
|115|DeleteAppConfig|`appPublicId: Nanoid<16, Base62Alphabet>`||Delete app configuration|false|
|116|EditAppConfig|`appPublicId: Nanoid<16, Base62Alphabet>`, `callBackUrl: Option<String>`, `minPasswordLength: Option<i32>`, `requiredPasswordChars: Option<String>`|`appPublicId: Nanoid<16, Base62Alphabet>`, `callBackUrl: String`, `minPasswordLength: i32`, `requiredPasswordChars: String`|Edit app configuration|false|
|117|GetAppSecurityRules|`appPublicId: Nanoid<16, Base62Alphabet>`|`appPublicId: Nanoid<16, Base62Alphabet>`, `minPasswordLength: i32`, `requiredPasswordChars: String`|Get security rules contained within current app's configuration|false|
|118|SetLogLevel|`logLevel: Option<LogLevel>`|`logLevel: LogLevel`|Set log level at runtime|false|

## authEndpoints Server
ID: 20
### Endpoints
|Code|Name|Parameters|Response|Description|FE Facing|
|-----------|-----------|----------|--------|-----------|-----------|
|200|ApiKeyConnect|`appApiKey: String`|||false|
|201|AuthorizedConnect|`accessToken: String`|||true|

## beCallbackApi Server
ID: 21
### Endpoints
|Code|Name|Parameters|Response|Description|FE Facing|
|-----------|-----------|----------|--------|-----------|-----------|
|210|ReceiveToken|`token: String`, `username: String`, `userPubId: Nanoid<16, Base62Alphabet>`||Backend receives auth tokens, happens after login|false|
|211|ReceiveUserInfo|`userPubId: Nanoid<16, Base62Alphabet>`, `username: String`, `appPubId: Option<Nanoid<16, Base62Alphabet>>`, `token: Option<String>`||Backend receives user info with optional token, happens after new user signs up. Platform app also receives this so that it can maintain records of app users, in which case Token will be set to None|false|
|212|ReceiveUserDeleted|`userPubId: Nanoid<16, Base62Alphabet>`, `appPubId: Option<Nanoid<16, Base62Alphabet>>`||Backend receives notification when a user is deleted or banned. App should clean up all user data and invalidate tokens.|false|
