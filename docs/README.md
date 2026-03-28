
# API Reference

## Structs/Datamodels

```rust

```
---

## Enums

```rust
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
|10|Signup|`appPublicId: Uuid`, `username: String`, `password: String`|`accessToken: String`|Frontend creates new user account.|true|
|12|SubmitUsername|`appPublicId: Uuid`, `username: String`|`expiresAt: i64`|Step 1: Frontend submits username during auth flow.|true|
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
|111|CreateAppConfig|`appPublicId: Uuid`, `callBackUrl: String`|`appPublicId: Uuid`, `createdAt: i64`, `appApiKey: String`, `minPasswordLength: i32`, `requiredPasswordChars: String`|Platform can create new apps|false|
|112|BanUser|`userPublicId: Uuid`||Ban a user|false|
|113|EditUser|`userPublicId: Uuid`, `newStatus: UserStatus`|`userPublicId: Uuid`, `newStatus: UserStatus`|Edit user status|false|
|114|DeleteUser|`appPublicId: Uuid`, `userPublicId: Uuid`||Delete a user|false|
|115|DeleteAppConfig|`appPublicId: Uuid`||Delete app configuration|false|
|116|EditAppConfig|`appPublicId: Uuid`, `callBackUrl: Option<String>`, `minPasswordLength: Option<i32>`, `requiredPasswordChars: Option<String>`|`appPublicId: Uuid`, `callBackUrl: String`, `minPasswordLength: i32`, `requiredPasswordChars: String`|Edit app configuration|false|
|117|GetAppSecurityRules|`appPublicId: Uuid`|`appPublicId: Uuid`, `minPasswordLength: i32`, `requiredPasswordChars: String`|Get security rules contained within current app's configuration|false|

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
|210|ReceiveToken|`token: String`, `username: String`, `userPubId: Uuid`||Backend receives auth tokens, happens after login|false|
|211|ReceiveUserInfo|`userPubId: Uuid`, `username: String`, `appPubId: Option<Uuid>`, `token: Option<String>`||Backend receives user info with optional token, happens after new user signs up. Platform app also receives this so that it can maintain records of app users, in which case Token will be set to None|false|
