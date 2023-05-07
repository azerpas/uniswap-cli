#!/bin/bash

# Define your arguments here
RPC="https://mainnet.infura.io/v3/"
CHAIN_ID="1"
AMOUNT_TO_APPROVE="1000000"
AMOUNT_TO_SWAP="500000"
FETCH_DECIMALS="true"
TOKEN_IN="0xdAC17F958D2ee523a2206206994597C13D831ec7"
TOKEN_OUT="0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
WEBHOOK="your_discord_webhook_url"

# Path to the uniswap-cli script
SCRIPT_PATH="/path/to/your/uniswap-cli/script"
# e.g SCRIPT_PATH="$HOME/uniswap-cli-darwin"

# Define the timestamp file path
TIMESTAMP_FILE="$HOME/.uniswap-cli/last_run"

# Get the current timestamp
CURRENT_TIME=$(date +%s)

# Check if the timestamp file exists and read the last execution timestamp
if [ -f "$TIMESTAMP_FILE" ]; then
    LAST_RUN=$(cat "$TIMESTAMP_FILE")
else
    LAST_RUN=0
fi

# Time interval in seconds (1 week)
INTERVAL=$((7 * 24 * 60 * 60))

# If the time since the last run is greater than or equal to the interval, run the script
if [ $((CURRENT_TIME - LAST_RUN)) -ge $INTERVAL ]; then
    # Launch the script with the terminal window "popping up"
    osascript <<EOD
tell application "Terminal"
    activate
    tell application "System Events" to tell process "Terminal" to keystroke "n" using command down
    do script "$SCRIPT_PATH --rpc $RPC --chain-id $CHAIN_ID --amount-to-approve $AMOUNT_TO_APPROVE --amount-to-swap $AMOUNT_TO_SWAP --fetch-decimals $FETCH_DECIMALS --token-in $TOKEN_IN --token-out $TOKEN_OUT --webhook $WEBHOOK" in selected tab of the front window
end tell
EOD
    # Update the timestamp file
    echo "$CURRENT_TIME" > "$TIMESTAMP_FILE"
fi
