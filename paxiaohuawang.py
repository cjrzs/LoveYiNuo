import re
import requests

respose=requests.get('http://www.xiaohuar.com/v/')
# print(respose.status_code)# 响应的状态码
# print(respose.content)  #返回字节信息
# print(respose.text)  #返回文本内容
urls=re.findall(r'class="items".*?href="(.*?)"',respose.text,re.S)  #re.S 把文本信息转换成1行匹配
url=urls[6]
result=requests.get(url)
mp4_url=re.findall(r'id="media".*?src="(.*?)"',result.text,re.S)[0]

video=requests.get(mp4_url)

with open('D:\\b.mp4','wb') as f:
    f.write(video.content)
