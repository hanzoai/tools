#!/bin/bash

export NODE_API_IP="0.0.0.0"
export NODE_IP="0.0.0.0"
export NODE_API_PORT="3690"
export NODE_WS_PORT="9551"
export NODE_PORT="9552"
export NODE_HTTPS_PORT="9553"
export PING_INTERVAL_SECS="0"
export RUST_LOG=debug,error,info
export STARTING_NUM_QR_PROFILES="1"
export STARTING_NUM_QR_DEVICES="1"
export FIRST_DEVICE_NEEDS_REGISTRATION_CODE="false"
export LOG_SIMPLE="true"
export SKIP_IMPORT_FROM_DIRECTORY="true"
export INSTALL_FOLDER_PATH="/dev/null"

# Add these lines to enable all log options
export LOG_ALL=1

if [ "$USE_DOCKER" = "true" ]; then
  # Run hanzo-node Docker container with environment variables
  docker run \
    -e NODE_API_IP \
    -e NODE_IP \
    -e NODE_API_PORT \
    -e NODE_WS_PORT \
    -e NODE_PORT \
    -e NODE_HTTPS_PORT \
    -e IDENTITY_SECRET_KEY \
    -e ENCRYPTION_SECRET_KEY \
    -e PING_INTERVAL_SECS \
    -e GLOBAL_IDENTITY_NAME \
    -e RUST_LOG \
    -e STARTING_NUM_QR_PROFILES \
    -e STARTING_NUM_QR_DEVICES \
    -e FIRST_DEVICE_NEEDS_REGISTRATION_CODE \
    -e LOG_SIMPLE \
    -e NO_SECRET_FILE \
    -e PROXY_IDENTITY \
    -e HANZO_TOOLS_RUNNER_DENO_BINARY_PATH \
    -e HANZO_TOOLS_RUNNER_UV_BINARY_PATH \
    -e LOG_ALL \
    -e INITIAL_AGENT_NAMES \
    -e INITIAL_AGENT_URLS \
    -e INITIAL_AGENT_MODELS \
    -e INITIAL_AGENT_API_KEYS \
    -e API_V2_KEY \
    -e EMBEDDINGS_SERVER_URL \
    -e SKIP_IMPORT_FROM_DIRECTORY \
    -e INSTALL_FOLDER_PATH \
    -p ${NODE_API_PORT}:${NODE_API_PORT} \
    -p ${NODE_WS_PORT}:${NODE_WS_PORT} \
    -p ${NODE_PORT}:${NODE_PORT} \
    -p ${NODE_HTTPS_PORT}:${NODE_HTTPS_PORT} \
    ${HANZO_NODE_IMAGE}
else
  # Download and run native binary
  if [[ "$(uname -s)" == "Darwin" && "$(uname -m)" == "arm64" ]]; then
      curl --location https://github.com/hanzoai/node/releases/download/v$HANZO_VERSION/hanzo-node-aarch64-apple-darwin.zip -o hanzo-node.zip
  elif [[ "$(uname -s)" == "Linux" && "$(uname -m)" == "x86_64" ]]; then
      curl --location https://github.com/hanzoai/node/releases/download/v$HANZO_VERSION/hanzo-node-x86_64-unknown-linux-gnu.zip -o hanzo-node.zip
  else
      echo "Unsupported platform"
      exit 1
  fi

  mkdir -p hanzo-node
  unzip -o hanzo-node.zip -d hanzo-node/
  cd hanzo-node
  ./hanzo-node
fi
