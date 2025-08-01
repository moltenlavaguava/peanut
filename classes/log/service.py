from __future__ import annotations

from classes.config.service import ConfigService
from classes.thread.service import ThreadService
from classes.event.service import EventService

import logging
import logging.handlers

# class for managing logging across differnet processes
class LoggingService():
    def __init__(self, configService:ConfigService, threadService:ThreadService, eventService:EventService):
        
        # ironic logging
        self.logger = logging.getLogger(__name__)
        
        # dependencies
        self.configService = configService
        self.threadService = threadService
        self.eventService = eventService
    
    # starts the logging service.
    def start(self):
        self._loggingQueue = self.threadService.createProcessQueue("Logging Queue")
        self._loggingThread = self.threadService.createThread(self._loggingManager, "Logging Manager")
        # subscribe to the program stop event
        self.eventService.subscribeToEvent("PROGRAM_CLOSE", self._eventCloseProgram)
        
    def _eventCloseProgram(self):
        # close the logging manager thread
        self._loggingQueue.put(None)
    
    def getLoggingQueue(self):
        return self._loggingQueue
    
    # most of this is taken from https://signoz.io/guides/how-should-i-log-while-using-multiprocessing-in-python/
    def _loggingManager(self):
        self.logger.info("Starting logging service.")
        
        # get necessary tools
        queue = self._loggingQueue
        root = logging.getLogger()
        root.setLevel(logging.getLogger().getEffectiveLevel())
        
        while True:
            try:
                # Get log record from the queue
                record = queue.get()
                # Check for the termination signal (None)
                if record is None:
                    break
                # Get the logger specified by the record and process the log message
                logger = logging.getLogger(record.name)
                logger.handle(record)
            except Exception as e:
                self.logger.error(f"An error occured while processing a record: {e}")
        self.logger.info("Shutting down threaded logging manager.")