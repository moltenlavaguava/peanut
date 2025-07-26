from __future__ import annotations

import PySide6.QtAsyncio as QtAsyncio
import asyncio
import threading
import logging

# manages threads and async utilities
class ThreadManager():
    def __init__(self):
        
        # logging management
        self.logger = logging.getLogger(__name__)
        
        self.logger.info("Starting thread manager.")
        
        # keeping track of all the existing threads
        self._threads: dict[str, threading.Thread] = {}
        self._threadEvents: dict[str, threading.Event] = {}
        
        # asnycio task management
        self._tasks = {}
    
    # checks to see if the current thread is the main one.
    def isThreadMainThread(self):
        return threading.current_thread() is threading.main_thread()
    
    # retrieves all current threads.
    def getThreads(self):
        return self._threads
    
    # get a thread with the specified name.
    def getThread(self, name:str):
        if name in self._threads:
            return self._threads[name]
        else:
            self.logger.warning(f"Thread not found with name {name}.")
    
    # waits for the specific thread to finish.
    def joinThread(self, name:str):
        if name in self._threads:
            self._threads[name].join()
            return
        else:
            self.logger.warning(f"Thread with name {name} not found when attempting to join.")
    
    # creates a threading.Thread with the given name <name>.
    def createThread(self, threadFunction:function, threadName:str):
        if threadName in self._threads:
            self.logger.warning(f"Thread already exists for name {threadName}. returning.")
            return
        thread = threading.Thread(target=threadFunction, name=threadName)
        thread.start()
        self._threads[threadName] = thread
        
    # returns whether or not the given thread event is set or not.
    def isThreadEventSet(self, eventName:str):
        if eventName in self._threadEvents:
            return self._threadEvents[eventName].is_set()
        else:
            self.logger.warning(f"Thread event not found for name {eventName}.")
            
    def setThreadEvent(self, eventName:str):
        if eventName in self._threadEvents:
            return self._threadEvents[eventName].set()
        else:
            self.logger.warning(f"Thread event not found for name {eventName}.")
        
    # creates a threading event with a given name.
    def createThreadEvent(self, eventName:str):
        if eventName in self._threadEvents:
            self.logger.warning(f"Thread event {eventName} already exists")
            return
        self._threadEvents[eventName] = threading.Event()
    
    # returns the given task if it exists.
    def getTask(self, name:str):
        if name in self._tasks:
            return self._tasks[name]
        else:
            self.logger.warning(f"Task with name {name} not found in the task list.")
            return
    
    # creates an asyncio task with the given name.
    def createTask(self, asyncFunction:asyncio._CoroutineLike, name:str):
        if name in self._tasks:
            self.logger.warning(f"Task name {name} already exists. returning")
            return
        if self.isThreadMainThread():
            # only use create_task in the main thread
            self._tasks[name] = asyncio.create_task(asyncFunction(), name=name)
        else:
            self.logger.info("createTask is being run in a different thread, using threadsafe version.")
            self._tasks[name] = asyncio.run_coroutine_threadsafe(asyncFunction(), name=name)
    
    # function for the main loop
    async def _mainLoop(self):
        self.logger.info("Booting up main loop.")
    
    # starts the main async loop.
    def startMainLoop(self):
        QtAsyncio.run(self._mainLoop(), keep_running=True, quit_qapp=False)
