from __future__ import annotations

import logging
import yt_dlp
import asyncio

# import yt-dlp's sanitation
from yt_dlp import utils as yt_dlp_utils

from .playlist import Playlist
from .track import PlaylistTrack

from classes.event.manager import EventManager

# downloader class for playlists
class PlaylistDownloader():
    
    def __init__(self, eventManager:EventManager):
        # initalize download options
       self.downloadOptions = {
        'extract_flat': True,  # Extract only basic information to make it faster
        'force_generic_extractor': True,
        'dump_single_json': True, # Request the data in JSON format
        'flat_playlist': True, # Extract only video IDs and titles from the playlist
        }
       # get logger
       self.logger = logging.getLogger(__name__)
       
       # dependencies 
       self.eventManager = eventManager
    
    # sanitize names using yt-dlp's method
    def _sanitizeFilename(self, txt:str):
        return yt_dlp_utils._utils.sanitize_filename(txt, restricted=True)
    
    # downloads the given playlist.
    async def downloadPlaylist(self, playlist:Playlist, downloadOptions):
        entries = playlist.getEntries()
        # replace playlist name with actual name
        downloadOptions["outtmpl"] = downloadOptions["outtmpl"].replace("%(playlist_title)s", playlist.getName())
        tasks = []
        with yt_dlp.YoutubeDL(downloadOptions) as ydl:
            # download video (+ logic that allows for shuffling)
            while True:
                for entry in entries:
                    if shuffleDownloadingPlaylistRequest or stopProcessEvent.is_set(): break
                    if entry["downloaded"]: continue
                    info = await downloadVideo(ydl, entry["url"])
                    path = ydl.prepare_filename(info) 
                    # convert file to specified format (async) and get the length of the file
                    task = asyncio.create_task(processAudioFileAsync(path, options["outputConversionExtension"], entry), name=f"Audio Conversion: {entry['displayName']}")
                    tasks.append(task)
                    savePlaylistFile(playlist)
                if shuffleDownloadingPlaylistRequest:
                    print("[Playlist] Request made to recheck (shuffle) downloading playlist.")
                    shuffleDownloadingPlaylistRequest = False
                    savePlaylistFile(playlist)
                elif stopProcessEvent.is_set():
                    print("[Playlist] Request recieved to stop playlist downloader.")
                    break
                else:
                    break
        # wait for everything to finish
        await asyncio.gather(*tasks)
        if stopProcessEvent.is_set():
            print("[Shutdown] Playlist downloader stopping.")
        else:
            print("[Playlist] Done downloading playlist..")
            playlist.setDownloaded(True)
        # save the file
        savePlaylistFile(playlist)
    
    # downloads the information for each track in the playlist. NOT supported by async functions.
    def initalizePlaylist(self, playlist:Playlist):
        playlistURL = playlist.getPlaylistURL()
        with yt_dlp.YoutubeDL(self.downloadOptions) as ydl:
            try:
                tracks = []
                info_dict = ydl.extract_info(playlistURL, download=False)
                playlist.setName(self._sanitizeFilename(info_dict["title"]))
                playlist.setDisplayName(info_dict["title"])
                if 'entries' in info_dict:
                    index = 0
                    for track in info_dict['entries']:
                        if track and 'url' in track:
                            index += 1
                            tracks.append(PlaylistTrack(track["url"], self._sanitizeFilename(track["title"]), track["title"], index))
                playlist.setTracks(tracks)
                playlist.setLength(len(tracks))
                playlist.setDownloaded(False)
                self.logger.info(f"Successfully finished initalizing playlist '{playlist.getDisplayName()}'")
            except Exception as e:
                self.logger.error(f"Error extracting playlist info: {e}")