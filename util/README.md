## utility programs ⚔️

#### lava loop signing

In many situations these may not make sense, but perhaps for automated push based system for system accounts,
something like this may be a useful reference.

- onion-knight - shell loop, signs based on arguments passed in - threadable - 3.17 hour cycle
- loyalist-server - rust loop, hard coded signing of $USER/.ssh/peak.pub with key in $USER/.ssh/dormant - 23 hour cycle

Both of these programs run along with the private CA, not on the final system user host. That is part of the security
model is that we can separate the CA keys from the user and server systems to isolated PKI systems, and if a user or
server key is stolen, it will only be valid with a cert for a short period. 
