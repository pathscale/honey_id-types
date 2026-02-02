
# publicApiConnection Server
ID: 0
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
|0|PublicConnect|||Initiates a websocket connection session with that permits access to endpoints with the UserRole::Public role|

# publicAuthApi Server
ID: 1
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
|10|Signup|appPublicId, username, password|accessToken|Frontend creates new user account.|
|12|SubmitUsername|appPublicId, username|expiresAt|Step 1: Frontend submits username during auth flow.|
|13|SubmitPassword|password|accessToken|Step 2: Frontend submits password to complete HoneyAuth login. Session is per connection. Returns tokens and token metadata.|

# platformApiKeyConnection Server
ID: 10
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
|100|PlatformConnect|platformApiKey||Handles platform API Key login to initiate the connection session between Honey API Backend and this server|

# platformApi Server
ID: 11
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
|111|CreateAppConfig|appPublicId, callBackUrl|appPublicId, createdAt, appApiKey, minPasswordLength, requiredPasswordChars|Platform can create new apps|
|112|BanUser|userPublicId||Ban a user|
|113|EditUser|userPublicId, newStatus|userPublicId, newStatus|Edit user status|
|114|DeleteUser|appPublicId, userPublicId||Delete a user|
|115|DeleteAppConfig|appPublicId||Delete app configuration|
|116|EditAppConfig|appPublicId, minPasswordLength, requiredPasswordChars|appPublicId, minPasswordLength, requiredPasswordChars|Edit app configuration|
|117|GetAppSecurityRules|appPublicId|appPublicId, minPasswordLength, requiredPasswordChars|Get security rules contained within current app's configuration|

# authEndpoints Server
ID: 20
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
|200|ApiKeyConnect|appApiKey|||
|201|AccessTokenConnect|accessToken|||
|202|GetAppPublicId||appPublicId|Returns `appPublicId` which will be used for auth flows|

# tokenApi Server
ID: 21
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
|210|ReceiveToken|token, username, userPubId||Backend receives auth tokens, happens after login or signup|
|211|ReceiveUserInfo|userPubId, username, token||Backend receives user info with token, happens after new user signs up.|
