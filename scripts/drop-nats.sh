#!/bin/bash
set -e

NATS_URL=${NATS_URL:-"nats://localhost:4222"}

echo "Deleting NATS JetStream stream..."

# Delete the stream, which also deletes associated consumers
if nats --server="$NATS_URL" stream info USER_EVENTS &> /dev/null; then
  echo "Deleting stream 'USER_EVENTS'..."
  nats --server="$NATS_URL" stream delete USER_EVENTS --force
else
  echo "Stream 'USER_EVENTS' does not exist."
fi

echo "NATS JetStream stream deleted successfully!"
