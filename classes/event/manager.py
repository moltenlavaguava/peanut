# functions as the main "event bus" for the program
from __future__ import annotations
from typing import Callable

import logging

class EventManager():
    def __init__(self):
        # setup logger
        self.logger = logging.getLogger(__name__)
        self._events: dict[str, list[Callable]] = {}
    
    # get the current list of events.
    def getEvents(self):
        return self._events
    
    # create an event that can be attached to.
    def addEvent(self, name:str):
        # make sure the event doesn't exist already
        if name in self.getEvents():
            self.logger.warning(f"Failed to create event with name '{name}': event already exists.")
            return
        self._events[name] = []
    
    # triggers the current event, passing in any relevant arguments.
    def triggerEvent(self, name:str, *args, **kwargs):
        # make sure the event actually exists
        events = self.getEvents()
        if not name in events:
            self.logger.warning(f"Failed to trigger event '{name}': event not found")
            return
        # call every callable
        for callable in events[name]:
            callable(*args, **kwargs)
    
    # subscribes a given callable to a given event name.
    def subscribeToEvent(self, name:str, callable:Callable):
        # make sure event exists
        events = self.getEvents()
        if not name in events:
            self.logger.error(f"Failed to subscribe to event '{name}': event does not exist")
            return
        # make sure that the callable isn't already in there
        if callable in events[name]:
            self.logger.warning(f"Failed to subscribe to event '{name}': callable is already subscribed")
            return
        currentIndex = len(events[name])
        events[name].append(callable)
        return currentIndex
        