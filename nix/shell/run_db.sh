set -e 

TMPDIR=$(mktemp -d /tmp/pgdata.XXXXXX)
PGDATA="$TMPDIR/.pgdata"
mkdir -p "$PGDATA"

if [ ! -d "$PGDATA/base" ]; then
  initdb -D "$PGDATA" --username=postgres --auth=trust
fi

postgres -D "$PGDATA" -k "$PWD"
