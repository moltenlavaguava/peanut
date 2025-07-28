from __future__ import annotations

import logging
import yt_dlp
import asyncio
from pathlib import Path
import os
import subprocess
import mutagen

# import yt-dlp's sanitation
from yt_dlp import utils as yt_dlp_utils

from .playlist import Playlist
from .track import PlaylistTrack

from classes.event.service import EventService
from classes.thread.service import ThreadService

# downloader class for playlists
class PlaylistDownloader():
    
    def __init__(self, eventService:EventService, threadService:ThreadService):
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
       self.eventService = eventService
       self.threadService = threadService
    
    # sanitize names using yt-dlp's method
    def _sanitizeFilename(self, txt:str):
        return yt_dlp_utils._utils.sanitize_filename(txt, restricted=True)
    
    # download a given video.
    def _downloadVideo(self, ydl: yt_dlp.YoutubeDL, url:str):
        info = ydl.extract_info(url, download=True)
        return info
    
    # converts an audio file to the desired extension
    def _convertAudioFile(self, filePath:str, newPath:str, ffmpegPath:str):
        # making new path with extension. extension must have dot
        args = [ffmpegPath, "-i", filePath, "-vn", "-y", "-ar", str(44100), "-ac", str(2), "-b:a", "192k", newPath]
        result = subprocess.run(args, text=True, check=True, capture_output=True)
        # delete old file
        os.remove(filePath)
    
    def _getAudioFileLength(self, path):
        return mutagen.File(path).info.length
    
    # converts an audio file to the desired extension and gets the length of the file.
    def _processAudioFile(self, filePath:str, extension:str, track:PlaylistTrack, ffmpegPath:str):
        self.logger.info(f"Processing {filePath}...")
        # get new path
        newPath = str(Path(filePath).with_suffix(extension))
        # convert
        self._convertAudioFile(filePath, newPath, ffmpegPath)
        # self._convertAudioFile(filePath, newPath, ffmpegPath)
        # get the length of the audio
        length = self._getAudioFileLength(newPath)
        track.setLength(length)
        self.logger.info(f"Finished processing {filePath}")
        # mark entry as finished
        track.setDownloaded(True)
    
    # downloads the given playlist. should be run in a thread as to not block the main gui.
    def downloadPlaylist(self, playlist:Playlist, downloadOptions, stopDownloadEvent, outputExtension:str, ffmpegPath:str):
        tracks = playlist.getTracks()
        # replace playlist name with actual name
        downloadOptions["outtmpl"] = downloadOptions["outtmpl"].replace("%(playlist_title)s", playlist.getName())
        with yt_dlp.YoutubeDL(downloadOptions) as ydl:
            for track in tracks:
                if stopDownloadEvent.is_set():
                    self.logger.info(f"Request received to stop playlist download for '{playlist.getName()}'.")
                    break
                if track.getDownloaded():
                    continue
                self.logger.info(f"Downloading {track.getDisplayName()}")
                info = self._downloadVideo(ydl, track.getVideoURL())
                self.logger.info(f"Done downloading {track.getVideoURL()}")
                path = ydl.prepare_filename(info)
                # convert file to specified format (async) and get the length of the file
                self._processAudioFile(path, outputExtension, track, ffmpegPath=ffmpegPath)
            if stopDownloadEvent.is_set():
                self.logger.info("Playlist downloader stopping.")
            else:
                self.logger.info("Done downloading playlist.")
                playlist.setDownloaded(True)
    
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