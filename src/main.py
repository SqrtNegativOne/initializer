import webview
from backend.backend import Api
import os

def get_html_path():
    html_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'frontend', 'index.html')
    return f"file:///{html_path}".replace('\\', '/')

if __name__ == '__main__':
    api = Api()
    window = webview.create_window('Genesis', get_html_path(), js_api=api, fullscreen=True)
    api.set_window(window)
    webview.start()
