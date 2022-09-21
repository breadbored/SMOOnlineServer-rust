# Super Mario Odyssey Online Multiplayer Server v0.0.3

## Unofficial macOS/Linux/Windows Rust Port of [Sanae6/SmoOnlineServer](https://github.com/Sanae6/SmoOnlineServer) for the [Super Mario Online Mod](https://github.com/CraftyBoss/SuperMarioOdysseyOnline) on Nintendo Switch

## How to build

1. [Follow the official Rust guide on installing rust on your platform](https://www.rust-lang.org/tools/install)
1. `cd` into the project directory from your terminal
1. Run `cargo fetch` to install the dependencies
1. Run `cargo build` to build the project
1. Run `./target/debug/smo-rusty-online` to start the server

Alternatively, there is a Docker container:

1. Install Docker
1. Just run `docker compose up`

If your Docker source hasn't rolled `compose` into `docker`:

1. Install Docker
1. Install Docker-Compose
1. Run `docker-compose up` with the hyphen

## Road Map

Because this is starting as a port, these are the features that need to be implemented to be compatible with the current version of SMO Online:

- [x] TCP Sockets
- [x] Packet Serialization
- [x] Packet Deserialization
- [ ] Client (in progress; needs to serialize packets)
- [ ] Server (in progress; can send and receive data; needs to init Clients)
- [ ] Command Handler
- [ ] Minimal Discord Integration, if any

Features I would like to add to the road map would be:

- [ ] Refactor the server and fork the SMO Online mod to support UDP packets with a TCP channel for state sync only
- [x] Docker container & Docker compose
- [ ] Full Discord Integration as the authors of the official server continue to work on it

## What it looks like currently

Running the server and connecting via the [Super Mario Online Mod](https://github.com/CraftyBoss/SuperMarioOdysseyOnline) on Nintendo Switch currently creates a character with your name and it follows you around with a small delay. This is a result of the TCP Sockets currently echoing back the data it receives until the command handler and client/server are finished.


https://user-images.githubusercontent.com/5916026/190936853-84890ec2-5fd9-4d5d-bb1c-e7f3a255476b.mp4

