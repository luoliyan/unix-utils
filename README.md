Collection of useful tools for working with Unix derivatives. FreeBSD- and OpenBSD-centric.

Some of these have both Python and Rust implementations. As a general rule, the Python versions will be more complete and the Rust ones more performant. Take your pick.

All code is released under the ISC license.

compare-installed
=================

Tested only on FreeBSD, but potentially useful on other BSD platforms. Takes a text file with a list of ports in port origin format (e.g. www/firefox) and compares against installed system packages. Output is those ports not on the list, as well as their dependent ports (if any). Helps in maintaining a list of ports for automated install, as well as identifying leaf packages that are no longer in use.

set-permissions
===============

Essentially a combined `chmod` and `chown`, but able to selectively set the execute permission bit based on whether the file is executable. I tend to maintain file permission of 600 on home directories, whereas 644 is more common. Allows a recursive `chmod` that maintains executable status. The Python implementation should be reasonably cross-platform; for Rust, `/usr/share/misc/magic` is hardcoded, so good luck.

update-bootcode
===============

Only applicable to ZFS on FreeBSD, and probably hardware dependent (assumes `ada` disk driver, among other things). On the correct set up, will identify the correct partitions to write updated bootcode, display some information, ask for permission, then write it. Only tested on one system, with `zroot` mirrored across two disks. Has proved useful after a `zpool upgrade`. Edit to meet your personal needs.

examine-syslog
==============

Somewhat useful in identifying hang points in the FreeBSD boot process, but could do with a lot of polish. Attempts to flag long delays between timestamps in `/var/log/messages` and average these out to eliminate once-off delays.
