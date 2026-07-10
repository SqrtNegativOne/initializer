import pytest
import sys
import os

# Add src to the sys.path
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '../src')))

from backend.backend import Api

def test_initial_state():
    api = Api()
    assert api.get_current_task() is None

def test_start_genesis():
    api = Api()
    api.start_routine("genesis")
    task = api.get_current_task()
    assert task['type'] == 'N'
    assert task['text'] == 'Complete mobile checklist.'
    assert task['index'] == 0

def test_complete_task_advances():
    api = Api()
    api.start_routine("genesis")
    api.complete_task(0)
    task = api.get_current_task()
    assert task['type'] == 'L'
    assert task['text'] == 'Launch Notion.'
    assert task['index'] == 1

def test_complete_last_task():
    api = Api()
    api.start_routine("genesis")
    # There are 7 tasks (index 0 to 6)
    for i in range(7):
        api.complete_task(i)
    assert api.get_current_task() is None # Finished
