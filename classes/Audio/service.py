from __future__ import annotations

from classes.event.service import EventService
from classes.playlist.service import PlaylistService

import logging

# manages various audio functions.
class AudioService():
    def __init__(self, eventService:EventService, playlistService:PlaylistService):
        
        # logging
        self.logger = logging.getLogger(__name__)
        
        # dependencies
        self.eventService = eventService
        self.playlistService = playlistService