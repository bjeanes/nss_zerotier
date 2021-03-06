# nss_zerotier

A NSS module to allow host lookup from joined ZeroTier networks

This is just getting started, but the idea is that it will return results only to `*.zt` requests under the following logic:

* [ ] `<nodeid>.zt` - the host with `<nodeid>` as its Node ID in any network to which the current machine belongs.
* [ ] `<host>.zt` - the first member found with `<host>` as its name in any network to which the current machine belongs.
  **NOTE**: The "name" in ZeroTier Central must be a valid hostname (which
  ZeroTier Central does not enforce).
* [ ] `<host>.<network>.zt` - the first member found in `<network>` with `<host>` as its name.
  **NOTE**: The "names" in ZeroTier Central must both form a valid hostname in
  combination (which ZeroTier Central does not enforce).
* [ ] `localhost.zt` - special case to return the addresses for the current machine (maybe, still thinking on this)

It will read the ZT token from the following places, in the following order:

1. [ ] `ZEROTIER_TOKEN` environment variable
1. [ ] `$HOME/.zeroTierOneAuthToken`, if present and readable by current user
1. [ ] `/var/lib/zerotier-one/authtoken.secret`, if present and readable by current user

Other than the environment variable, these are the same places that `zerotier-one` and `zerotier-cli` check. I am not
currently aware of any environment variable which the ZeroTier tools check, but if there is one, this will be adjusted
to align.

## Installation

``` sh-session
$ make
   Compiling ...
   Compiling nss_zerotier v0.1.0 (/home/bjeanes/Code/nss_zerotier)
    Finished release [optimized] target(s) in 11.08s
$ sudo make install
install -m755 -d /usr/lib/
install -m644 target/release/libnss_zerotier.so /usr/lib/libnss_zerotier.so.2
ldconfig -n /usr/lib/
```

* [x] **TODO**: provide a `Makefile`
* [ ] **TODO**: pre-compile releases and attach to [GitHub Releases](https://github.com/bjeanes/nss_zerotier/releases)
* [ ] **TODO**: package as an AUR for ArchLinux

## Usage

After installation, you should be able to query the database using `getent`:

``` sh-session
$ getent -s zerotier hosts
10.144.17.130	d37fa4928f.zt vorpal.home.zt vorpal.zt
10.144.119.0	124df28d26.zt tumtum.home.zt tumtum.zt
10.144.70.159	68f86f5d48.zt nas.home.zt nas.zt
$ getent -s zerotier hosts nas.zt
10.144.70.159	68f86f5d48.zt nas.home.zt nas.zt
```

To enable the `zerotier` NSS module system-wide, you'll need to edit `/etc/nsswitch.conf` and edit the `hosts` entry.
For example:

``` diff
# ...
passwd: files mymachines systemd
group: files mymachines systemd
shadow: files
publickey: files
-hosts: files mymachines myhostname resolve [!UNAVAIL=return] dns
+hosts: files mymachines myhostname zerotier resolve [!UNAVAIL=return] dns
networks: files
# ...
```

## Compatibility

This should (once implemented) work on any operating system which uses [Name Service Switch](https://en.wikipedia.org/wiki/Name_Service_Switch),
but I am not yet familiar with any potential variations in the glibc representation for the required callbacks and
data structures.

In theory, that means Linux and BSD flavours should be easily supported. I personally am building this on an ArchLinux
machine.

I know macOS has some facilities for custom resolvers but I also know it is not based on NSS. Potentially, this could
eventually offer some equivalent there but it is not currently a goal.

## Alternatives

* https://github.com/jjsarton/zt_nss
* https://github.com/uxbh/ztdns

## License

MPL. See `LICENSE` file.
