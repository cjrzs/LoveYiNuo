import requests
import json

url = "http://192.168.133.90:7800/chat/knowledge_base_chat"

payload = json.dumps({
  "query": "量化怎么做",
  "knowledge_base_name": "samples"
})
headers = {
  'Content-Type': 'application/json'
}

response = requests.request("POST", url, headers=headers, data=payload)

print(response.text)
