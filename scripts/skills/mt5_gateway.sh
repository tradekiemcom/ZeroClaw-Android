#!/bin/bash
# ==============================================================================
# 💹 ZeroClaw MT5 Web Gateway Connector (v13.0)
# ------------------------------------------------------------------------------
# Sử dụng REST API để kết nối với trạm trung chuyển MT5 Web Terminal.
# ==============================================================================

MT5_URL="${MT5_GATEWAY_URL:-http://localhost:8080}"
MT5_TOKEN="${MT5_GATEWAY_TOKEN:-your_token}"

function get_account_info() {
    curl -s -X GET "$MT5_URL/api/account" \
         -H "Authorization: Bearer $MT5_TOKEN"
}

function open_order() {
    local symbol=$1
    local type=$2 # buy/sell
    local lots=$3
    
    curl -s -X POST "$MT5_URL/api/order" \
         -H "Content-Type: application/json" \
         -H "Authorization: Bearer $MT5_TOKEN" \
         -d "{\"symbol\":\"$symbol\", \"type\":\"$type\", \"lots\":$lots}"
}

# Xử lý tham số CLI
case "$1" in
    "--info")
        get_account_info
        ;;
    "--trade")
        open_order "$2" "$3" "$4"
        ;;
    *)
        echo "Sử dụng: $0 --info | --trade [SYMBOL] [TYPE] [LOTS]"
        ;;
esac
