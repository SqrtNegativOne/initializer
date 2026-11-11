import sys
import os

# Prevent pythonw.exe from crashing when writing to stdout/stderr
log_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'app.log')
if sys.stdout is None:
    sys.stdout = open(log_path, 'a')
if sys.stderr is None:
    sys.stderr = open(log_path, 'a')

import webview
from backend.backend import Api

def get_html_path():
    html_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'frontend', 'index.html')
    return f"file:///{html_path}".replace('\\', '/')

if __name__ == '__main__':
    api = Api()
    window = webview.create_window(
        'Genesis',
        get_html_path(),
        js_api=api,
        fullscreen=True,
        frameless=True,
        transparent=True,
        easy_drag=False,
    )
    api.set_window(window)
    webview.start()
