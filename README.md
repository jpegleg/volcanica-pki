# volcanica-pki 🌋

Some ed25519 ssh key and certificate generation tools.

- rust dalek ed25519 and os rng key generation

- vanity public key feature available but not used by default

- gnupg asymmetric identity and AES256 for storage

- 1 day ssh certificates

## installation (for both server and client)

Cargo is required to compile volcanica-pki. The binary can also be copied without compiling but the install script does compile with cargo by default unless the binary is in the `$pwd` as `volcanica-pki`, in which case the `$(pwd)/volcanica-pki` will be used for the install to `/usr/local/bin/`.

```
bash install_volcanica
```

## usage

On the signing server, generate a new user access CA:

```
new_volcano_ca $(cat /etc/machine-id) mypgpgthing@mypgpgkey.thingsetc
```

The second argument can be anything desired for the key ID. In the example we use the `machine-id` value.

Sign the host_key used by each server (send the public host key over to the PKI server):

```
volcanica_sign $(cat /etc/machine-id) myservers1.net,myserver2.net,myserver3.net ssh_host_ed25519.pub
```

The host_key can be generated just like a client, but has to be configured to be used in the `/etc/ssh/sshd_config`.

On a client side, generating a new key pair:

```
new_volcano_client $(cat /etc/machine-id) someuserpgpkey@placeandstuff
```

The public key can be sent to the PKI server to then sign (will be prompted for gpg password to the private key):

```
volcanica_sign $(cat /etc/machine-id) myuser "$clientmachineid".pub
```

## additional setup

Ensure the servers have the appropriate `/etc/ssh/sshd_config` and that the client side has the appropriate `~/.ssh/known_hosts` and `/etc/ssh/known_hosts` to trust the ca public key.
