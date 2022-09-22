# volcanica-pki 🌋

Some ed25519 ssh key and certificate generation tools.

- rust dalek ed25519 and os rng key generation

- vanity public key feature available but not used by default

- gnupg asymmetric identity and AES256 for storage

- 1 day ssh user certificates default

- 1 year ssh host key certificate default

## optional gpg and lava mode

The more secure but less automated way uses gpg (gnupg2) to wrap private keys in symmetric AES256 via asymmetric pgp identity keys. This requires the keyring containing the gpg secret key and password for it to decrypt. Alternative to that, there is the wrappers that start with `lava_` which do not protect the ed25519 private keys with gpg. For fully automated solutions, the `lava_` versions will be used, or otherwise modified with "automatable" key access.

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
volcanica_sign_host $(cat /etc/machine-id) myservers1.net,myserver2.net,myserver3.net ssh_host_ed25519.pub
```

The host certificate is signed with ssh-keygen using the decrypted key file, but has to be configured to be used in the `/etc/ssh/sshd_config`.

On a client side, generating a new key pair:

```
new_volcano_client $(cat /etc/machine-id) someuserpgpkey@placeandstuff
```

The public key can be sent to the PKI server to then sign (will be prompted for gpg password to the private key):

```
volcanica_sign_user $(cat /etc/machine-id) myuser "$clientmachineid".pub
```

## additional setup

Ensure the servers have the appropriate `/etc/ssh/sshd_config` and that the client side has the appropriate `~/.ssh/known_hosts` and/or `/etc/ssh/known_hosts` to trust the ca public key.
