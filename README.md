Bevy plugins for authentication and chat. See roadmap in plan.txt.
1) Websockets lib echo server and client - done, can send and receive messages
2) Create websocket client plugin for bevy - should connect to given server and provide interface for sending and receiving messages - in progress.
Fixing bugs and adding functionality.
3) Create websocket server plugin - exposes port for websocket connection, provides interface for reading incoming messages and sending messages
prob 2 and 3 can share some code - for receiving and sending msgs - posponed, don't need bevy ws server now.
4) Create UI for login screen and chat. Will start after backend for auth and chat is working.