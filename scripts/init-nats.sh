#!/bin/bash
set -e

NATS_URL=${NATS_URL:-"nats://localhost:4222"}

echo "Initializing NATS JetStream..."

# Wait for NATS to be ready
until nats --server="$NATS_URL" server check connection; do
  echo "Waiting for NATS..."
  sleep 2
done

# Create streams
if ! nats --server="$NATS_URL" stream info USER_EVENTS &> /dev/null; then
  echo "Creating stream 'USER_EVENTS'..."
  nats --server="$NATS_URL" stream add USER_EVENTS \
    --subjects "user.>" \
    --storage file \
    --retention limits \
    --max-age 24h \
    --replicas 1 \
    --defaults
else
  echo "Stream 'USER_EVENTS' already exists."
fi

# Create consumers
if ! nats --server="$NATS_URL" consumer info USER_EVENTS world-user-registered &> /dev/null; then
  echo "Creating consumer 'world-user-registered'..."
  nats --server="$NATS_URL" consumer add USER_EVENTS world-user-registered \
    --pull \
    --filter "user.registered" \
    --ack explicit \
    --max-deliver 3 \
    --defaults
else
  echo "Consumer 'world-user-registered' already exists."
fi

if ! nats --server="$NATS_URL" consumer info USER_EVENTS session_service_consumer &> /dev/null; then
  echo "Creating consumer 'session_service_consumer'..."
  nats --server="$NATS_URL" consumer add USER_EVENTS session_service_consumer \
    --pull \
    --filter "user.logged.*" \
    --ack explicit \
    --max-deliver 3 \
    --defaults
else
    echo "Consumer 'session_service_consumer' already exists."
fi

GAME_SERVERS=("star_map" "cloning" "iris" "liberte")

for name in "${GAME_SERVERS[@]}"; do
  consumer_name="game_server_${name}"

  echo "Checking consumer '${consumer_name}'..."
  if ! nats --server="$NATS_URL" consumer info USER_EVENTS "$consumer_name" &>/dev/null; then
    echo "Creating consumer '${consumer_name}'..."
    nats --server="$NATS_URL" consumer add USER_EVENTS "$consumer_name" \
      --pull \
      --filter "user.logged.*" \
      --ack explicit \
      --max-deliver 3 \
      --defaults
  else
    echo "Consumer '${consumer_name}' already exists."
  fi

  echo
done

nats --server="$NATS_URL" consumer ls USER_EVENTS


echo "NATS JetStream initialized successfully!"
