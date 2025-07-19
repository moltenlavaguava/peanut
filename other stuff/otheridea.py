from selenium import webdriver
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.chrome.service import Service
from selenium.webdriver.common.by import By

import time

chromeOptions = Options()
# chrome_options.add_argument("--headless")
# chrome_options.add_argument("--no-sandbox")  # Recommended for headless environments
# chrome_options.add_argument("--disable-dev-shm-usage") # Recommended for headless environments
# chrome_options.add_argument("--autoplay-policy=no-user-gesture-required") # Allows audio to play without user interaction

# Remove the default --mute-audio argument
chromeOptions.add_experimental_option("excludeSwitches", ["mute-audio"])
chromeOptions.add_extension("adblock.crx")

# driver = webdriver.Chrome(options=chromeOptions)
driverPath = "chromedriver.exe"

chromeService = Service(driverPath)
browser =  webdriver.Chrome(service=chromeService, options=chromeOptions)

browser.get("https://www.youtube.com/results?search_query=never+gona+give+you+up")

time.sleep(360)