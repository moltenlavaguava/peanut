from __future__ import annotations

import PySide6.QtAsyncio as QtAsyncio
import asyncio
import threading
import logging
import multiprocessing

# manages threads and async utilities
class ThreadService():
    def __init__(self):
        
        # logging management
        self.logger = logging.getLogger(__name__)
        
        self.logger.info("Starting thread service.")
        
        # keeping track of all the existing threads
        self._threads: dict[str, threading.Thread] = {}
        self._threadEvents: dict[str, threading.Event] = {}
        
        # asnycio management
        self._tasks = {}
        self._asyncioEvents = {}
        
        # process management
        self._processes: dict[str, multiprocessing.Process] = {}
        
        # main loop. manually maintained
        self._mainLoopAlive: bool = False
        self._mainLoopObject: asyncio.BaseEventLoop = None
    
    # THREADING
    
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
    def createThread(self, threadFunction:function, threadName:str, *args, **kwargs):
        if threadName in self._threads:
            self.logger.warning(f"Thread already exists for name {threadName}. returning.")
            return
        thread = threading.Thread(target=threadFunction, name=threadName, *args, kwargs=kwargs)
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
        event = threading.Event()
        self._threadEvents[eventName] = event
        return event
    
    # COROUTINES
    
    # returns the given task if it exists.
    def getTask(self, name:str):
        if name in self._tasks:
            return self._tasks[name]
        else:
            self.logger.warning(f"Task with name {name} not found in the task list.")
            return
    
    def getAsyncioEvents(self):
        return self._asyncioEvents
    
    def getAsyncioEvent(self, name:str):
        events = self.getAsyncioEvents()
        if name in events:
            return events[name]
        else:
            self.logger.warning(f"Failed to get asyncio event with name '{name}': event does not exist")
    
    def resetAsyncioEvent(self, name:str):
        events = self.getAsyncioEvents()
        if name in events:
            events[name].clear()
        else:
            self.logger.warning(f"Failed to reset asyncio event '{name}': event does not exist")
    
    def setAsyncioEvent(self, name:str):
        events = self.getAsyncioEvents()
        if name in events:
            events[name].set()
        else:
            self.logger.warning(f"Failed to set asyncio event with name '{name}': event does not exist")
    
    # creates an asyncio event with the given name.
    def createAsyncioEvent(self, name:str):
        events = self.getAsyncioEvents()
        # make sure it doesn't already exist
        if name in events:
            self.logger.warning(f"Failed to create asyncio event with name '{name}': event already exists")
            return
        events[name] = asyncio.Event()
    
    # returns the current event loop if it exists.
    def getEventLoop(self):
        # make sure the loop is currently alive
        if self.isEventLoopAlive():
            return self._mainLoopObject
        else:
            self.logger.warning("Failed to retrieve event loop: event loop is not running")
            return
    
    def isEventLoopAlive(self):
        return self._mainLoopAlive
    
    # run a given async function in asyncio's loop
    def runInExecutor(self, func:callable, *args):
        # retrieve the current event loop
        loop = self.getEventLoop()
        if not loop: return
        # run in executor
        return loop.run_in_executor(None, func, *args)
    
    # creates an asyncio task with the given name.
    def createTask(self, asyncFunction:asyncio._CoroutineLike, name:str):
        if name in self._tasks:
            self.logger.warning(f"Task name {name} already exists. returning")
            return
        if self.isThreadMainThread():
            # only use create_task in the main thread
            task = asyncio.create_task(asyncFunction, name=name)
            self._tasks[name] = task
            return task
        else:
            self.logger.info("createTask is being run in a different thread, using threadsafe version.")
            task = asyncio.run_coroutine_threadsafe(asyncFunction, name=name, loop=self.getEventLoop())
            self._tasks[name] = task
            return task
    
    # PROCESSES
    
    # returns all processes (excluding the current process)
    def getProcesses(self):
        return self._processes
    
    # creates a process with the given name.
    def createProcess(self, processFunction:callable, name:str, start:bool, *args, **kwargs):
        # make sure it doesn't already exist
        processes = self.getProcesses()
        if name in processes:
            self.logger.warning(f"Failed to create process '{name}': process already exists")
            return
        process = multiprocessing.Process(target=processFunction, name=name, args=args, kwargs=kwargs)
        processes[name] = process
        if start:
            process.start()
    
    # main loop
    
    # function for the main loop
    async def _mainLoop(self):
        self.logger.info("Booting up main loop.")
        
        # set variables
        self._mainLoopAlive = True
        self._mainLoopObject = asyncio.get_event_loop()
        
        # keep main loop alive
        while True:
            await asyncio.sleep(1)
        
        self._mainLoopAlive = False
        self.logger.info("Closing main loop.")
    
    # starts the main async loop.
    def startMainLoop(self):
        QtAsyncio.run(self._mainLoop(), keep_running=True, quit_qapp=False)
