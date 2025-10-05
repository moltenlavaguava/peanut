from __future__ import annotations

import logging
import yt_dlp
import asyncio
from pathlib import Path
import os
import subprocess
import requests
from multiprocessing.synchronize import Event
from multiprocessing import Queue
from ytmusicapi import YTMusic
from PIL import Image

# import yt-dlp's sanitation
from yt_dlp import utils as yt_dlp_utils

from .playlist import Playlist
from .track import PlaylistTrack

# downloader class for playlists
class PlaylistDownloader():
    
    def __init__(self, logger):
        # initalize download options
       self.downloadOptions = {
        'extract_flat': True,  # Extract only basic information to make it faster
        'force_generic_extractor': True,
        'dump_single_json': True, # Request the data in JSON format
        'flat_playlist': True, # Extract only video IDs and titles from the playlist
        }
       # get logger
       self.logger = logger
       self.ytmusic = YTMusic()
    
    # utility
    
    # sanitize names using yt-dlp's method
    def _sanitizeFilename(self, txt:str):
        return yt_dlp_utils._utils.sanitize_filename(txt, restricted=True)
    
    # file handling
    
    # squares an image. used to turn thumbnails into album covers.
    def _squareImage(self, filePath:str):
        img = Image.open(filePath)
        width, height = img.size
        minDim = min(width, height)
        
        left = (width - minDim) / 2
        top = (height - minDim) / 2
        right = (width + minDim) / 2
        bottom = (height + minDim) / 2

        box = (left, top, right, bottom)
        cropimg = img.crop(box)
        cropimg.save(filePath)
    
    # downloading
    
    def _downloadThumbnail(self, url:str, outputLocation:str):
        response = requests.get(url, stream=True)
        response.raise_for_status()
        with open(outputLocation, "wb") as f:
            for chunk in response.iter_content(chunk_size=8192):
                f.write(chunk)
    
    # download a given video.
    def _downloadVideo(self, ydl: yt_dlp.YoutubeDL, url:str):
        info = ydl.extract_info(url, download=True)
        return info
    
    # converts an audio file to the desired extension (unused, handled by yt-dlp)
    # def _convertAudioFile(self, filePath:str, newPath:str, ffmpegPath:str):
    #     # making new path with extension. extension must have dot
    #     args = [ffmpegPath, "-i", filePath, "-vn", "-y", "-ar", str(44100), "-ac", str(2), "-b:a", "192k", newPath]
    #     result = subprocess.run(args, text=True, check=True, capture_output=True)
    #     # delete old file
    #     os.remove(filePath)
    
    # searches yt music for album data for the given track.
    def _getAlbumData(self, searchTerm:str, trackLength:float, maxVariation:float, playlist:Playlist):
        results = self.ytmusic.search(searchTerm, "songs", None, limit=1, ignore_spelling=True)
        if trackLength == 0:
            self.logger.warning("Track length is 0 seconds, this probably is not correct")
    
        if results:
            mainResult = results[0]
            # if no album, return
            if not mainResult["album"]: self.logger.debug(f"Album Data Request failed for term '{searchTerm}': No album entry present"); return None, None, None, None
            # if this wasn't the artist's own upload, return
            if not mainResult["videoType"] == "MUSIC_VIDEO_TYPE_ATV": self.logger.debug(f"Album Data Request failed for term '{searchTerm}': no videoType entry present"); return None, None, None, None, None
            # if the track length is more than <maxVariation> seconds different than the yt video, return
            if abs(mainResult["duration_seconds"] - trackLength) > maxVariation: 
                self.logger.debug(f"Album Data Request failed for term '{searchTerm}': track lengths do not match up ({mainResult['duration_seconds']}s on yt music vs. {trackLength}, with max variation of {maxVariation}s)"); return None, None, None, None
            albumName = self._sanitizeFilename(mainResult["album"]["name"])
            albumDisplayName = mainResult["album"]["name"]
            albumID = mainResult["album"]["id"]
            artistName = None # will be assigned shortly
            # check to see if the album art is already downloaded
            albums = playlist.getAlbums()
            imageURL = None
            if not albumName in albums:
                albumData = self.ytmusic.get_album(albumID)
                imageURL = albumData["thumbnails"][-1]["url"]
                # download go brrrrrrrrrr
                # self.logger.debug(f"Downloading album image for album '{albumDisplayName}' via youtube music search.")
                artistName = ", ".join(d["name"] for d in albumData["artists"])
                # save the album to prevent redownloading
                playlist.addAlbumEntry(albumName, {"artist": artistName, "display name:": albumDisplayName})
            else:
                artistName = albums[albumName]["artist"]
            return albumName, albumDisplayName, artistName, mainResult["title"], imageURL
        else:
            self.logger.debug(f"Album Data Request failed for term '{searchTerm}': no results found with search term"); return None, None, None, None
    
    # converts an audio file to the desired extension and gets the length of the file. (unused)
    # def _processAudioFile(self, filePath:str, extension:str, track:PlaylistTrack, ffmpegPath:str):
    #     self.logger.info(f"Processing {filePath}...")
    #     # get new path
    #     newPath = str(Path(filePath).with_suffix(extension))
    #     # convert
    #     self._convertAudioFile(filePath, newPath, ffmpegPath)
    #     self.logger.info(f"Finished processing {filePath}")
    #     # mark entry as finished
    #     track.setDownloaded(True)
    
    # downloads the given playlist. should be run in a thread as to not block the main gui.
    def downloadPlaylist(self, playlist:Playlist, downloadOptions, outputExtension:str, 
                         stopEvent:Event, responseQueue:Queue, albumCoverOutput:str, useYoutubeMusicAlbums:bool, maxVariation:int, 
                         startIndex:int, selectIndex, selectLock, idRequestConnection, thumbnailOutput:str,
                         downloadedData: dict[int, bool]):
        name = playlist.getName()
        self.logger.debug(f"Start index: {startIndex}")
        # cache if this download was successful or not
        downloadPlaylistSuccess = True
        stopDownloading = False
        # replace playlist name with actual name
        # downloadOptions["outtmpl"] = downloadOptions["outtmpl"].replace("%(playlist_title)s", name)
        if not playlist.getThumbnailDownloaded():
            self._downloadThumbnail(playlist.getThumbnailURL(), os.path.join(thumbnailOutput, f"{name}.jpg"))
            playlist.setThumbnailDownloaded(True)
        # with yt_dlp.YoutubeDL(downloadOptions) as ydl:
        while not stopDownloading:
            self.logger.debug("Starting loop")
            tracks = playlist.getTracks()
            for index in range(startIndex, len(tracks)):
                track = tracks[index]
                
                if stopEvent.is_set():
                    stopDownloading = True
                    break

                # select index things: check to see if the value actually changed
                with selectLock:
                    if selectIndex.value != -1:
                        # break out of the loop and restart the download
                        break

                if downloadedData[track.getID()]:
                    continue
                
                # download + processing
                self.logger.info(f"Downloading video '{track.getDisplayName()}'.")
                responseQueue.put({"action": "TRACK_DOWNLOAD_START", "track": track, "playlistName": playlist.getName(), "downloadIndex": index})
                
                # download options for only this track
                localDownloadOptions = downloadOptions.copy()
                localDownloadOptions["outtmpl"] = localDownloadOptions["outtmpl"].replace("%(id)s", str(track.getID()))
                with yt_dlp.YoutubeDL(localDownloadOptions) as ydl:
                    info = None
                    try:
                        info = self._downloadVideo(ydl, track.getVideoURL())
                    except Exception as e:
                        # at least one track failed to download; restart at the end of the playlist and try agin
                        self.logger.warning(f"Failed to download track '{track.getDisplayName()}'; skipping and moving on")
                        downloadPlaylistSuccess = False 
                        responseQueue.put({"action": "TRACK_DOWNLOAD_DONE", "track": track, "playlistName": name, "downloadIndex": index, "success": False})
                        continue

                # # attempt to download video. will try multiple times. if max attempts is -1, then it will try indefinitely.
                # attemptCount = 0
                # info: dict|None = None
                # success = True
                # while True:
                #     if (attemptCount == maxDownloadAttempts) and (maxDownloadAttempts != 1):
                #         # ran out of attempts
                #         success = False
                #         break
                #     try:
                #         info = self._downloadVideo(ydl, track.getVideoURL())
                #         if not info:
                #             # attempt failed, try again
                #             attemptCount += 1
                #         else:
                #             # attempt succeeded, break
                #             break
                #     except Exception as e:
                #         self.logger.error(f"An unknown error occured while attempting to download the track '{track.getName()}': {e}")
                #         downloadPlaylistSuccess = False # mark the playlist as not being fully downloaded
                #         attemptCount += 1
                # if not success:
                #     self.logger.error(f"Failed to download track '{track.getName()}': ran out of attempts")
                #     responseQueue.put({"action": "TRACK_DOWNLOAD_DONE", "track": track, "playlistName": name, "downloadIndex": index, "success": False})
                #     continue        
                
                # path = ydl.prepare_filename(info)
                # convert file to specified format (not getting the track length atm)
                # self._processAudioFile(path, outputExtension, track, ffmpegPath=ffmpegPath)
                
                # attempt to get music data
                autogenVideo = True # whether or not the album data was handled via auto-generated video
                try:
                    albumName = self._sanitizeFilename(info["album"])
                    albumDisplayName = info["album"]
                    artistName = ", ".join(info["artists"])
                    idRequestConnection.send([(albumName, "ALBUM",)])
                    idData = idRequestConnection.recv()
                    track.setAlbumID(idData[0]["id"])
                    # check to see if the album was already downloaded
                    if not idData[0]["downloaded"]:
                        albumImageURL = info["thumbnails"][-1]["url"]
                        self.logger.debug(f"Downloading album image for track '{albumDisplayName}' via auto-generated video.")
                        imgPath = os.path.join(albumCoverOutput, f"{idData[0]['id']}.jpg")
                        self._downloadThumbnail(albumImageURL, imgPath)
                        self._squareImage(imgPath)
                        # square the image
                    track.setAlbumName(albumName)
                    track.setAlbumDisplayName(albumDisplayName)
                    track.setArtistName(artistName)
                except KeyError:
                    autogenVideo = False
                
                if not autogenVideo:
                    # try to get an album cover from youtube music
                    # self.logger.debug(f"Info: {track.toDict()}")
                    trackLength = track.getLength()
                    if trackLength == 0:
                        self.logger.warning(f"track length directly from track is 0. Why? Track dict: {track.toDict()}")
                    self.logger.debug("Attempting to retrieve album data..")
                    albumName, albumDisplayName, artistName, trackName, imageURL = self._getAlbumData(searchTerm=track.getDisplayName(), trackLength=trackLength, maxVariation=maxVariation, playlist=playlist)
                    self.logger.debug("Data retrieval done.")
                    if albumName:
                        # request id data
                        idRequestConnection.send([(albumName, "ALBUM",), (trackName, "TRACK",)])
                        self.logger.debug("recieving id data..")
                        idData = idRequestConnection.recv() # wait for response
                        self.logger.debug("id data recieved.")
                        track.setAlbumName(albumName)
                        track.setAlbumDisplayName(albumDisplayName)
                        track.setArtistName(artistName)
                        track.setDisplayName(trackName)
                        track.setAlbumID(idData[0]["id"])

                        # todo: change id system to use audio fingerprinting
                        # track.setID(idData[1])
                        # download the album cover
                        if imageURL and not idData[0]["downloaded"]:
                            self.logger.debug(f"Downloading album cover for '{albumDisplayName}' via ytmusic serach")
                            self._downloadThumbnail(imageURL, os.path.join(albumCoverOutput, f"{idData[0]['id']}.jpg"))
                        else:
                            self.logger.debug(f"Not downloading album cover for '{albumDisplayName}'; cover already downloaded or no cover exists")
            
                # signal the completion of the track download
                downloadedData[track.getID()] = True
                responseQueue.put({"action": "TRACK_DOWNLOAD_DONE", "track": track, "playlistName": name, "downloadIndex": index, "success": True})
            if not stopEvent.is_set():

                # check to see if a selection was made
                with selectLock:
                    self.logger.debug(f"Select index value: {selectIndex.value}")
                    if selectIndex.value != -1:
                        # restart the download at the specified index
                        startIndex = selectIndex.value
                        downloadPlaylistSuccess = True
                        self.logger.debug(f"Restarting the playlist downloader at index {selectIndex.value}.")
                        selectIndex.value = -1
                    elif downloadPlaylistSuccess:
                        # verify download
                        undownloaded = False
                        for track in playlist.getTracks():
                            if not downloadedData[track.getID()]:
                                undownloaded = True
                                break
                        if not undownloaded:
                            self.logger.info("Done downloading playlist.")
                            stopDownloading = True
                        else:
                            startIndex = 0
                    else:
                        # restart the playlist download process
                        downloadPlaylistSuccess = True
    
    # downloads the information for each track in the playlist. NOT supported by async functions.
    def initalizePlaylist(self, playlist:Playlist):
        playlistURL = playlist.getPlaylistURL()
        with yt_dlp.YoutubeDL(self.downloadOptions) as ydl:
            tracks = []
            info_dict = ydl.extract_info(playlistURL, download=False)
            playlist.setName(self._sanitizeFilename(info_dict["title"]))
            playlist.setDisplayName(info_dict["title"])
            if 'entries' in info_dict:
                index = 0
                for track in info_dict['entries']:
                    if track and 'url' in track:
                        index += 1
                        duration = track["duration"]
                        if duration == 0:
                            self.logger.warning(f"Track duration for {track['title']} is 0s which probably is not correct")
                        playlistTrack = PlaylistTrack(videoURL=track["url"], name=self._sanitizeFilename(track["title"]), displayName=track["title"], index=index, length=duration)
                        tracks.append(playlistTrack)
            playlist.setTracks(tracks)
            playlist.setLength(len(tracks))
            playlist.setThumbnailURL(info_dict["thumbnails"][-1]["url"])