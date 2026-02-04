
# publicApiConnection Server
ID: 0
## Endpoints
|Method Code|Method Name|Parameters|Response|Description| FE Facing |
|-----------|-----------|----------|--------|-----------|-----------|
|0|PublicConnect|||Initiates a websocket connection session with that permits access to endpoints with the UserRole::Public role|true|

# publicAuthApi Server
ID: 1
## Endpoints
|Method Code|Method Name|Parameters|Response|Description| FE Facing |
|-----------|-----------|----------|--------|-----------|-----------|
|10|Signup|appPublicId, username, password|accessToken|Frontend creates new user account.|true|
|12|SubmitUsername|appPublicId, username|expiresAt|Step 1: Frontend submits username during auth flow.|true|
|13|SubmitPassword|password|accessToken|Step 2: Frontend submits password to complete HoneyAuth login. Session is per connection. Returns tokens and token metadata.|true|

# platformApiKeyConnection Server
ID: 10
## Endpoints
|Method Code|Method Name|Parameters|Response|Description| FE Facing |
|-----------|-----------|----------|--------|-----------|-----------|
|100|PlatformConnect|platformApiKey||Handles platform API Key login to initiate the connection session between Honey API Backend and this server|false|

# platformApi Server
ID: 11
## Endpoints
|Method Code|Method Name|Parameters|Response|Description| FE Facing |
|-----------|-----------|----------|--------|-----------|-----------|
|111|CreateAppConfig|appPublicId, callBackUrl|appPublicId, createdAt, appApiKey, minPasswordLength, requiredPasswordChars|Platform can create new apps|false|
|112|BanUser|userPublicId||Ban a user|false|
|113|EditUser|userPublicId, newStatus|userPublicId, newStatus|Edit user status|false|
|114|DeleteUser|appPublicId, userPublicId||Delete a user|false|
|115|DeleteAppConfig|appPublicId||Delete app configuration|false|
|116|EditAppConfig|appPublicId, minPasswordLength, requiredPasswordChars|appPublicId, minPasswordLength, requiredPasswordChars|Edit app configuration|false|
|117|GetAppSecurityRules|appPublicId|appPublicId, minPasswordLength, requiredPasswordChars|Get security rules contained within current app's configuration|false|

# authEndpoints Server
ID: 20
## Endpoints
|Method Code|Method Name|Parameters|Response|Description| FE Facing |
|-----------|-----------|----------|--------|-----------|-----------|
|200|ApiKeyConnect|appApiKey|||false|
|201|AuthorizedConnect|accessToken|||true|

# tokenApi Server
ID: 21
## Endpoints
|Method Code|Method Name|Parameters|Response|Description| FE Facing |
|-----------|-----------|----------|--------|-----------|-----------|
|210|ReceiveToken|token, username, userPubId||Backend receives auth tokens, happens after login or signup|false|
|211|ReceiveUserInfo|userPubId, username, token||Backend receives user info with token, happens after new user signs up.|false|
