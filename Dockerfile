# base runner image. Nothing but curl and bash
from alpine as runner
run apk update
run apk add curl bash
user 1000
run mkdir -p /app

# test installation from local tarball
from runner as install-local
copy --from=builder build/bootstrap-*.tgz build/
run env ./bootstrap.sh 

# test installation from github
from runner as install-remote
