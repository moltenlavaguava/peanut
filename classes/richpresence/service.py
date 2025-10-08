from __future__ import annotations

import logging
import asyncio
import pypresence
import traceback
import os
import time
from pypresence import ActivityType, StatusDisplayType

from classes.event.service import EventService
from classes.thread.service import ThreadService
from classes.audio.service import AudioService
from classes.id.service import IDService

from classes.playlist.track import PlaylistTrack

# for integrating with discord's rich presence
class DRPService():
    def __init__(self, eventService:EventService, threadService:ThreadService, audioService:AudioService, idService:IDService):
        
        self.logger = logging.getLogger(__name__)
        self.eventService = eventService
        self.threadService = threadService
        self.audioService = audioService
        self.idService = idService
        
        self._CLIENT_ID = "1424985710724648981"
    
    def start(self, enabled:bool):
        # create thread
        self._rpcThread = self.threadService.createThread(self._runRichPresence, "DRP_HANDLER", enabled=enabled)
        self._enabled = enabled # globaL status of enabled to compare against local version
        # handle shutdown
        self.eventService.subscribeToEvent("PROGRAM_CLOSE", self._eventProgramClose)
        
    def enable(self):
        self.logger.info("Enabling Discord Rich Presence.")
        self._enabled = True
        
    def disable(self):
        self.logger.info("Disabling Discord Rich Presence.")    
        self._enabled = False
    
    def _eventProgramClose(self):
        self.logger.debug("closing")
        if self._rpcLoop and self._rpcLoop.is_running():
            self.logger.info("Scheduling RPC task cancellation and loop shutdown.")
            
            def shutdown():
                # Cancel the task. This will raise CancelledError inside the task.
                if self._rpcTask and not self._rpcTask.done():
                    self._rpcTask.cancel()
                
                # Stop the loop. This will cause run_forever() to return.
                self._rpcLoop.stop()

            # Schedule the shutdown to happen on the loop's own thread
            self._rpcLoop.call_soon_threadsafe(shutdown)
    
    # main method to carry out rich presence calls.
    def _runRichPresence(self, enabled):
        self.logger.debug("Starting RPC Thread.")
        # create new event loop just for this thread
        asyncio.set_event_loop_policy(None)
        
        # manually create the loop regardless of pyside
        defaultPolicy = asyncio.DefaultEventLoopPolicy()
        self._rpcLoop = defaultPolicy.new_event_loop()
        asyncio.set_event_loop(self._rpcLoop)
        
        async def run(rpc, enabled):
            
            # records the last payload sent to discord
            lastPayloadTime = 0
            
            # if an update should be forced
            forceUpdate = True
            
            lastTrack: PlaylistTrack|None = None
            lastTrackStartTime: int|None = None
            lastPaused: bool = None
            
            try:
                startTime = int(time.time())
                endTime = startTime + 42000
                await rpc.connect()
                while True:
                    # if a request was made to enable rich presence, honor it
                    if self._enabled and not enabled:
                        enabled = True
                        forceUpdate = True
                    # if the last request was made more than 15 seconds ago, consider sending something
                    if time.time() - lastPayloadTime > 15:
                        # if a request has been made to disable rich presence
                        if enabled and not self._enabled:
                            # pause communication
                            await rpc.clear()
                            enabled = False
                            lastPayloadTime = time.time()
                        else:
                            
                            # collect data
                            
                            currentTrack = self.audioService.getCurrentTrack()
                            currentTime = time.time()
                            currentTrackStartTime = None
                            currentPaused = self.audioService.getPaused()
                            currentPlayback = self.audioService.getCurrentPlayback()
                            
                            if currentTrack and currentPlayback:
                                currentTrackStartTime = int(currentTime - currentPlayback.curr_pos)
                            
                            if (forceUpdate) or ((currentTrackStartTime != lastTrackStartTime) or (currentTrack != lastTrack) or (currentPaused != lastPaused)):
                                self.logger.debug("Sending rich presence update..")
                                # gather information
                                
                                albumID = None
                                if currentTrack:
                                    albumID = self.idService.getAlbumIDFromTrackID(currentTrack.getID())
                                    
                                artistName, albumName, albumURL = None, None, None
                                if albumID:
                                    albumData = self.idService.getAlbumDataFromID(albumID)
                                    artistName = albumData["artist"]
                                    albumName = albumData["displayName"]
                                    albumURL = albumData["imageURL"]
                                
                                details = ""
                                state = ""
                                startTime = None
                                endTime = None
                                
                                if currentPlayback:
                                    details = f"{currentTrack.getDisplayName()}{' (paused)' if currentPaused else ''}"
                                    state = f"{artistName if artistName else '(unknown artist)'}"
                                    if not currentPaused:
                                        startTime = currentTrackStartTime
                                        endTime = int(startTime + currentPlayback.duration)
                                    else:
                                        startTime = self.audioService.getPauseTime()
                                else:
                                    details = "browsing peanut"
                                    state = "maybe you should too :)"
                                
                                # send an update
                                await rpc.update(
                                activity_type=2, # "listening" activity
                                status_display_type=0, # displays application name
                                # note: files do not work, but urls do
                                small_image=f"{albumURL if albumURL else 'peanut'}",
                                small_text=f"{albumName if albumName else 'hi :)'}",
                                details=details,
                                state=state,
                                buttons=[
                                    {"label": "checkout peanut", "url": "https://github.com/moltenlavaguava/peanut"},
                                    {"label": "click 4 cat", "url": "https://aleatori.cat/random"},
                                ],
                                start=startTime,
                                end=endTime,
                                )
                                
                                # updating data
                                lastTrack = currentTrack
                                lastTrackStartTime = currentTrackStartTime
                                lastPaused = currentPaused
                                
                                # stop forcing an update
                                forceUpdate = False
                                lastPayloadTime = time.time()
                    await asyncio.sleep(3)
            except asyncio.CancelledError:
                self.logger.debug("rpc task cancelled.")
            except Exception as e:
                self.logger.error(f"An unknown error occured in the rpc task: {e}")
                self.logger.error(traceback.format_exc())
            finally:
                self.logger.debug("Closing rpc connection.")
                rpc.close()
        
        rpc = pypresence.AioPresence(self._CLIENT_ID, loop=self._rpcLoop)
        self._rpcTask = self._rpcLoop.create_task(run(rpc, enabled))
        try:
            self.logger.debug("starting rpc event loop")
            self._rpcLoop.run_forever()
        finally:
            self.logger.debug("rpc loop stopped. closing loop now")
            self._rpcLoop.close()