import subprocess
import os
import yaml
from pathlib import Path

# Locate the checklists directory relative to the src folder
CHECKLISTS_DIR = Path(__file__).parent.parent.parent / "checklists"

class Api:
    def __init__(self):
        self.current_checklist_name = None
        self.current_tasks = []
        self.current_task_index = 0
        self.window = None
        self.routines = [
            {"name": "genesis", "type": "checklist", "file": "calibration.yaml"},
            {"name": "speculatio", "type": "quit"},
            {"name": "expeditum", "type": "quick_job"}
        ]

    def set_window(self, window):
        self.window = window

    def unfullscreen_and_maximize(self):
        if self.window:
            self.window.toggle_fullscreen()
            self.window.maximize()

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
            if self.window:
                self.window.destroy()
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
        if self.window:
            self.window.destroy()

    def quit_app(self):
        if self.window:
            self.window.destroy()
