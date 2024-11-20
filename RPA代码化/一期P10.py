import random
import time

from selenium import webdriver
from selenium.webdriver.chrome.service import Service
from selenium.webdriver.common.by import By
import pyautogui
from selenium.webdriver.common.action_chains import ActionChains

# 设置 WebDriver 路径
driver_path = 'D:\\chromedriver-win64\\chromedriver.exe'
chrome_path = 'D:\\chrome-win64\\chrome.exe'
# 创建一个 WebDriver 实例
service = Service(executable_path=driver_path)
# 设置 Chrome 浏览器位置
options = webdriver.ChromeOptions()
options.binary_location = chrome_path

# 创建一个 Chrome 浏览器实例
driver = webdriver.Chrome(service=service, options=options)

# 打开网页
driver.get('https://loginmyseller.taobao.com/')

driver.implicitly_wait(1)
driver.switch_to.frame(driver.find_element(By.ID, "alibaba-login-box"))

input_element = driver.find_element(By.XPATH, value=r"//input[@id='fm-login-id']")
input_element.send_keys('绫致时装正品店:rpa')

# 定位输入框并输入内容
input_element = driver.find_element(By.ID, 'fm-login-password')
input_element.send_keys('Qwer1234')

button_element = driver.find_element(By.XPATH, r'//*[@id="login-form"]//button[text()="登录"]')
time.sleep(5)

try:
    actions = ActionChains(driver)

    driver.switch_to.frame(driver.find_element(By.XPATH, r'//iframe[@id="baxia-dialog-content"]'))
    slide_verify_element = driver.find_element(By.XPATH, value=r'//span[text()="向右滑动验证"]')
    slide_block_element = driver.find_element(By.XPATH, value=r'//span[@aria-label="滑块"]')
    print(slide_verify_element.rect, slide_block_element.rect)
    x_offset = slide_verify_element.rect['width'] - slide_block_element.rect['width']
    print('总移动距离：', x_offset)
    # actions.click_and_hold(slide_block_element).move_by_offset(x_offset, 0).release().perform()
    # 模拟拖放操作
    actions.click_and_hold(slide_block_element)  # 点击并保持开始拖动的元素
    # 下面这行是关键，如果有直接的像素距离需求，需要自己计算目标位置的偏移量
    # 随机速度控制
    for _ in range(50):
        # 随机暂停时间，模拟不同速度
        pause_time = random.uniform(0.01, 0.03)  # 暂停0.2秒到0.5秒之间
        time.sleep(pause_time)
        actions.move_to_element_with_offset(slide_block_element, x_offset / 50, 0)

    # 执行动作链
    actions.release().perform()

except Exception as e:
    print(e)
# 等待用户输入，按 Enter 后继续执行
input("Press Enter to continue...")



