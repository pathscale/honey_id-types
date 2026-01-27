
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
|10|StartAuth|appPublicId|loginConfig|Frontend initiates HoneyAuth authorization flow. Session is stored per connection.|
|11|Signup|username, password, agreedTos, agreedPrivacy|accessToken, refreshToken, idToken, tokenType, expiresIn, profileCallbackUrl|Frontend creates new user account during HoneyAuth signup. Session is per connection. Returns authorization code and profile callback endpoint.|
|12|SubmitUsername|username|expiresAt|Step 1: Frontend submits username during HoneyAuth flow. Session is per connection.|
|13|SubmitPassword|password|accessToken, refreshToken, idToken, tokenType, expiresIn|Step 2: Frontend submits password to complete HoneyAuth login. Session is per connection. Returns tokens and token metadata.|
|14|RefreshTokenExchange|refreshToken, appPublicId|accessToken, refreshToken, idToken, tokenType, expiresIn|Exchange refresh token for access tokens. Redirect URI handling is client-side; use WebSocket connection context for navigation.|
|15|TokenRevoke|token, tokenTypeHint, appPublicId, clientSecret|success|Revoke access or refresh tokens.|

# publicWaitlistApi Server
ID: 2
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
|20|AddWaitlistLead|name, telegram, whatsApp, description||Adds a lead to the waitlist.|

# publicKardVipApi Server
ID: 3
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
|30|KardAddWaitlistLead|name, telegram, whatsApp, description||Adds a lead to the waitlist.|

# apiKeyEndpoints Server
ID: 10
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
|100|ApiKeyConnect|appPublicId, appApiKey||Handles API Key login to initiate the AppApiKey connection session between App Backend and Honey Auth Server|

# honeyAuthAppApi Server
ID: 11
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
|110|TokenIntrospect|token, tokenTypeHint, appPublicId, appApiKey|active, scope, appPublicId, username, userPublicId, exp, iat|Backend validates HoneyAuth tokens (requires app API key).|
|111|SubscribeTokenRevocations|appPublicId, appApiKey|subscriptionId|Backend subscribes to user token revocation events. TODO. This is not implemented|
