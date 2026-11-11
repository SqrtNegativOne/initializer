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
    assert task is not None
    assert task['index'] == 0
    assert 'text' in task
    assert 'type' in task

def test_complete_task_advances():
    api = Api()
    api.start_routine("genesis")
    api.complete_task(0)
    if len(api.current_tasks) > 1:
        task = api.get_current_task()
        assert task['index'] == 1
    else:
        assert api.get_current_task() is None

def test_complete_all_tasks():
    api = Api()
    api.start_routine("genesis")
    for i in range(len(api.current_tasks)):
        api.complete_task(i)
    assert api.get_current_task() is None
