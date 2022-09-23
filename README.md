# volcanica-pki 🌋

Some ed25519 ssh key and certificate generation tools.

- hardened sshd_config to enforce certificate use

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

Optionally generate a separate ssh server CA. Them being different has tactical beneifts.

```
new_volcano_ca userseca root@lansegmentssomething.local
```

The first argument can be anything desired for the key ID. In the example we use the `machine-id` value.
The second argument is the gpg key id or key email.

Sign the host_key used by each server (send the public host key over to the PKI server):

```
volcanica_sign_host $(cat /etc/machine-id) myservers1.net,myserver2.net,myserver3.net ssh_host_ed25519.pub
```

Another example with a different naming scheme:

```
volcanica_sign_host prod_group100 myservers1.net,myserver2.net,myserver3.net ssh_host_ed25519.pub
```

The host certificate is signed with ssh-keygen using the decrypted key file, but has to be configured to be used in the `/etc/ssh/sshd_config`.

On a client side, generating a new key pair:

```
new_volcano_client $(cat /etc/machine-id) someuserpgpkey@placeandstuff
```

We encrypt the private key with a pgp key and write to a file name "$1".asc. The private key doesn't have to
touch the disk with the default method of key generation. The signing operation currently does expose the private key to the disk briefly. 
In `lava_` mode, the plaintext private key is written to disk during generation, and the renewal scripts use the name exactly rather than a "$1".asc version.


The public key can be sent to the PKI server (TBD on transmission approach there) to then sign (will be prompted for gpg password to the private key unless using lava mode):

```
volcanica_sign_user userseca myuser "$clientmachineid".pub
```

The input names are intentionally flexible in this system. Whatever is used as the first argument will be treated as "$1".asc,
representing the PKI key. The output of the sign is the certificate, which is not sensitive information that is meant to be published
to the needed parties.

Another example with a different naming approach:

```
volcanica_sign_user admin_group_root root "$clientmachineid".pub
```

## ssh with a certificate

The user side can specify both certificate and key on the command line like so:
```
ssh -i ~/.ssh/summerSalt-cert.pub -i ~/.ssh/summerSalt root@myserver
```


## additional setup

Ensure the servers have the appropriate `/etc/ssh/sshd_config` (see templates/server__sshd_config) and that the client side has the appropriate `~/.ssh/known_hosts` and/or `/etc/ssh/known_hosts` to trust the ca public key (see templates/client__ssh_known_hosts).

#### common patterns

- shorten the host certificate lifespan default from one year to two months and use lava mode on the hosts
- change the extensions used to customize SSH access features
- have the user renewal process used by humans require MFA including at least one cryptographic identity check, one central identity check, and one proof of hardware check
- have the user for automation less secure but renewed and rekeyed automatically each hour or so, renewal done only from restricted source and key use reduced to the approved automation jobs executions only


## coming next

Ansible and more support automation will be aded.

## warning 🐉 

If volcanica-pki is deployed, ssh keys will not work for access unless they have a valid certificate as per the sshd_config hardening.

Ensure that either console access is available or that you have access to the CAs used in the deployment to sign public keys for access!
