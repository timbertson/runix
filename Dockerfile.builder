# builder image, shares /nix so it needs to be running as the same user/group
from alpine
arg HOST_UID
run apk update
run apk add git
run adduser -D -u "${HOST_UID}" app
run ln -sfn /host-runix/nix/store /tmp/runix
