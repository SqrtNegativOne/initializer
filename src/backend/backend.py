import subprocess
import os
from pathlib import Path

import webview
import yaml

# Locate the checklists directory relative to the src folder
CHECKLISTS_DIR = Path(__file__).parent.parent.parent / "checklists"

OVERLAY_WIDTH = 380
OVERLAY_HEIGHT = 220
OVERLAY_MARGIN = 24

class Api:
    def __init__(self):
        self.current_checklist_name = None
        self.current_tasks = []
        self.current_task_index = 0
        self._window = None
        self.routines = [
            {"name": "genesis", "type": "checklist", "file": "calibration.yaml"},
            {"name": "speculatio", "type": "quit"},
            {"name": "expeditum", "type": "quick_job"}
        ]

    def set_window(self, window):
        self._window = window

    def unfullscreen_and_maximize(self):
        if self._window:
            self._window.toggle_fullscreen()
            self._window.maximize()

    def enter_checklist_mode(self) -> dict[str, str]:
        """Shrink the window into a translucent overlay docked to the top right."""
        if not self._window:
            return {"status": "error", "message": "Window unavailable"}

        if self._window.fullscreen:
            self._window.toggle_fullscreen()
            self._window.fullscreen = False

        self._window.resize(OVERLAY_WIDTH, OVERLAY_HEIGHT)
        screen = webview.screens[0]
        x = max(screen.width - OVERLAY_WIDTH - OVERLAY_MARGIN, OVERLAY_MARGIN)
        self._window.move(x, OVERLAY_MARGIN)
        self._window.on_top = True
        return {"status": "ok"}

    def get_routines(self):
        return self.routines

    def load_checklist(self, filename):
        filepath = CHECKLISTS_DIR / filename
        if filepath.exists():
            with open(filepath, 'r') as f:
                return yaml.safe_load(f)
        return []

    def start_routine(self, routine_name):
        routine = next((r for r in self.routines if r["name"] == routine_name), None)
        if not routine:
            return {"status": "error", "message": "Routine not found"}

        if routine["type"] == "checklist":
            self.current_checklist_name = routine["name"]
            self.current_tasks = self.load_checklist(routine.get("file", ""))
            self.current_task_index = 0
            return {"status": "ok", "type": "checklist"}
        elif routine["type"] == "quit":
            if self._window:
                self._window.destroy()
            return {"status": "ok", "type": "quit"}
        elif routine["type"] == "quick_job":
            self.current_checklist_name = routine["name"]
            self.current_tasks = []
            self.current_task_index = 0
            return {"status": "ok", "type": "quick_job"}
        return {"status": "error", "message": "Unknown type"}

    def get_current_task(self):
        if self.current_task_index < len(self.current_tasks):
            task = self.current_tasks[self.current_task_index].copy()
            task['index'] = self.current_task_index
            return task
        return None

    def complete_task(self, index):
        if index == self.current_task_index:
            task = self.current_tasks[self.current_task_index]
            if task['type'] == 'L':
                try:
                    target = task.get('target')
                    if target:
                        if target.endswith('.vbs'):
                            os.startfile(target)
                        else:
                            subprocess.Popen(f'"{target}"', shell=True)
                    else:
                        print("Error launching task: No target specified")
                except Exception as e:
                    print(f"Error launching {task.get('target', 'unknown')}: {e}")
            self.current_task_index += 1
            if self.current_task_index >= len(self.current_tasks):
                return {"status": "done"}
            return {"status": "next", "task": self.get_current_task()}
        return {"status": "error", "message": "Index mismatch"}

    def launch_everything_normally(self):
        print("Launching everything normally...")
        if self._window:
            self._window.destroy()

    def quit_app(self):
        if self._window:
            self._window.destroy()
