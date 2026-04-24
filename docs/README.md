# space

> Experimental event-driven network layer on top of the internet.

**space** is a general-purpose event network — an infrastructure layer on which any kind of application can be built.

---

## Concept

In physics, reality is not made of states. It is made of events — discrete changes propagating through spacetime. State is an approximation, a snapshot derived from accumulated events. An observer does not create reality; it records it.

This system applies that principle directly to networked communication.

All activity in the network is expressed as signed, immutable **Events**. A single central **Observer** records these events without authority or judgment, assigning each a globally ordered sequence number. Everything else — delivery, encryption, application logic — is the responsibility of **Participants**.

This model — **Observer and Participants** — is the core of this system. It was derived independently from physical principles, not from existing software patterns.

The Observer corresponds to the role of a physical law: it records what happened, impartially, without influencing the outcome. Participants are the actors. The Observer is never the actor.

Because there is exactly one Observer, there is exactly one timeline. Global consistency is not achieved through consensus algorithms — it is guaranteed by the structure of the system itself. This is the same reason physics requires a single reference frame to define a universal ordering of events: multiple observers produce multiple timelines, and consistency between them becomes impossible to guarantee.

---

## Identity

Identity in this network is a cryptographic key pair. Specifically, Ed25519.

The public key is the identity. The private key proves ownership of that identity by signing every Event the Participant emits. There are no usernames, no passwords, no accounts. Generating a key pair is generating an identity.

Any entity capable of holding a key pair and speaking the protocol is a Participant — a human using a browser, a Raspberry Pi, an IoT sensor, a bot, a server. The Observer itself holds a key pair. All Participants are equal.

---

## Security

Security is structural. It is not a layer added on top — it emerges from the design.

Impersonation is mathematically impossible without the private key. Tampering is detectable by signature verification. The Observer never needs to be trusted, because it cannot forge a signature it did not produce.

Payload encryption is optional and the responsibility of the Participant. The EventLog is effectively public. If content is sensitive, the Participant encrypts it before placing it in the payload. If not, it is plaintext. The network does not enforce privacy — mathematics does, when the Participant chooses to use it.

---

## Extensibility

Every Participant speaks the same protocol: sign an Event, receive a NetworkEvent. This uniformity is what makes the network infinitely extensible.

Adding a new capability means adding a new Participant type — a machine that listens for certain Events and responds to them. The Observer never changes. The Transport Layer never changes. The protocol never changes.

This is analogous to how the physical internet works: TCP/IP does not know what runs on top of it. This network does not know what applications run on top of it.

---

## Event Model

> Work in progress. The following is directional, not final.

```proto
syntax = "proto3";
package event;

message EncryptedPayload {
  bytes nonce      = 1;  // AES-256-GCM nonce
  bytes ciphertext = 2;  // AES-256-GCM encrypted data
  bytes key_hash   = 3;  // hash of the encryption key
}

message Event {
  bytes  sender_id  = 1;  // Ed25519 public key
  string namespace  = 2;  // e.g. "space.network.time"
  bytes  payload    = 3;
  bytes  signature  = 4;  // sign(sender_id + namespace + payload)
}

message NetworkEvent {
  Event  event              = 1;
  uint64 sequence_number    = 2;  // assigned by Observer
  int64  observer_timestamp = 3;  // assigned by Observer
}
```

The `namespace` field classifies Events and allows Participants to subscribe selectively. `space.*` is reserved. Everything else is open.

---

## Technology

Native components (Observer, infrastructure machines) are written in Rust as single binaries. The web client is React + Vite. Serialization is Protocol Buffers (`prost = "0.14.3"`). Browser Participants connect via WebSocket; machine Participants connect via QUIC. The EventLog is SQLite.

---

## Status

Early design phase. This document is a living specification.

`github.com/barn0w1/space`