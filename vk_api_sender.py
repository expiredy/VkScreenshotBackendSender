import os
from urllib import response
from dotenv import load_dotenv
load_dotenv(".env")

import requests


login =  os.environ.get("VK_LOGIN")
password = os.environ.get("VK_PASSWORD")
app_token = os.environ.get("VK_APP_TOKEN")
user_id = os.environ.get("VK_USER")


# requests_constructor = f"https://oauth.vk.com/authorize?client_id={user_id}&redirect_uri=https://oauth.vk.com/blank.html&scope=friends&response_type=token&v=5.131"
requests_constructor = f""


response = requests.get(requests_constructor)
print(response)