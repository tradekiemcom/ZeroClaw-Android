import requests

TOKEN = "QSIx4r7f4dPrHANV10xu9azUY2bds7fMVHbRDh9mQsc"
URL = f"https://api.spotware.com/connect/tradingaccounts?access_token={TOKEN}"

try:
    response = requests.get(URL)
    print(f"Status: {response.status_code}")
    print(response.json())
except Exception as e:
    print(f"Error: {e}")
