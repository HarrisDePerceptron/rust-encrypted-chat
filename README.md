# rust-encrypted-chat


## TODO 
- [ ] Websocket auth (token  cookies)
    * [x] Secure
    * CSRF?
    * [x] Bearer Token
    * [x] Cookie Token
- [x] DOT ENV File
- [ ] Enable TLS/WSS
- [x] Message UUIDs time based
- [ ] Database persistence
    * Redis
    * Mongo
- [ ] Scaling websockets (Stickty session?)
- [ ] Chat Implementation 
    * list rooms 
    * presence
    * create room
    * invite user to room
    * send message to room
    * list room messages
    * sending files and photos
    * E2E encryption

- Notifications (one way messages)
    * send a notifcation 
    * list notifications
    * send notification to a user
    * schedule a notification
    * Cancel  a notification
    * Send all notification
    * firebase notification provider (IOS, Android)