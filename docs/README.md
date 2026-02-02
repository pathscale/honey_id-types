
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

# authEndpoints Server
ID: 10
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
|100|ApiKeyConnect|appApiKey|||
|101|AccessTokenConnect|accessToken|||
|102|GetAppPublicId||appPublicId|Returns `appPublicId` which will be used for auth flows|

# tokenApi Server
ID: 11
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
|110|ReceiveToken|token, username, userPubId||Backend receives auth tokens, happens after login or signup|
|111|ReceiveUserInfo|userPubId, username, token||Backend receives user info with token, happens after new user signs up.|
