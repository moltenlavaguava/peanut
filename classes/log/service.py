from __future__ import annotations

from classes.config.service import ConfigService
from classes.thread.service import ThreadService

import logging
import logging.handlers

# class for managing logging across differnet processes
class LoggingService():
    def __init__(self, configService:ConfigService, threadService:ThreadService):
        
        # ironic logging
        self.logger = logging.getLogger(__name__)
        
        # dependencies
        self.configService = configService
        self.threadService = threadService
    
    # starts the logging service.
    def start(self):
        self._loggingQueue = self.threadService.createProcessQueue("Logging Queue")
        self._loggingThread = self.threadService.createThread(self._loggingManager, "Logging Manager")
    
    def getLoggingQueue(self):
        return self._loggingQueue
    
    # most of this is taken from https://signoz.io/guides/how-should-i-log-while-using-multiprocessing-in-python/
    def _loggingManager(self):
        self.logger.info("Starting logging service.")
        
        # get necessary tools
        queue = self._loggingQueue
        root = logging.getLogger()
        root.setLevel(self.configService.getLoggerOptions()["level"])
        
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