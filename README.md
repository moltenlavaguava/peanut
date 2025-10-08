# peanut 🥜

peanut is a YouTube-based video archiver and streamer. It supports the download of most YouTube videos, with support planned for other online services. 

peanut itself is developed with Python, and has an interactive GUI.

# Features

The main features of peanut currently are:
- Video downloading with conversion to audio (.ogg) files
- Offline audio playback with playlist controls
- User friendly GUI
- System wide hotkeys to control playback without interacting with the GUI
- Artist and Album data for most YouTube videos via YouTube Music

To see all upcoming and planned features, please direct your eyes to the latest release.

**Example usage:**

![Screenshot of program in use with a playlist loaded](/resources/example.png)

## Installation

**From Release**

To download peanut from a release, first locate the latest release. Then, choose the appropriate version for your system (currently peanut is only released for windows) and download it. Extract the resulting zip file, and run `peanut.exe`. Note: currently this project is not signed with a certificate, as they take both large amounts of time and money to fully establish for a program, but if you have too much money, just let me know ;). Because of this, you will likely see a window saying that your PC was protected. To work around this, simply click "more details" then "run anyway." If you do not trust that peanut is free of viruses, you are welcome to poke around in the source code yourself (but you won't find anything bad).

**From Source**

Requirements:
- Python installation (recommended to be Python >=3.11)
- Required Python packages
- Python virtual environment (highly recommended)
- Visual Studio Code (recommended for easier environment management)

First, clone the repository using git. To do this, first [download git](https://git-scm.com/downloads). Open a terminal window, navigate to the folder you want to be the parent for the repository folder, then run `git clone https://github.com/LavaMaster77/peanut`.

To install Python, go to [Python's official website and download the appropriate release](https://www.python.org/downloads/).

To setup the Python virtual environment, follow one of the two following steps:

*Visual Studio Code*

If you do not already have visual studio code installed, you can download it [here](https://code.visualstudio.com/download). Once you have it downloaded, press Ctrl+Shift+P (if on Windows) or Command+Shift+P (if on Mac). In the command palette, search for Python: Create Environment. Select `Venv`, and follow the steps presented (including selecting the python interpreter you have).

*Manual Environment Creation*

To manually create your virtual environment, run `python -m venv <enviornment name>` (or substitute `python` for whatever the name of your python executable is). Then, run `.\<enviornment name>\Scripts\activate` to enable the virtual enviornment for your current terminal session. Note that you will need to do this step every time you open a new terminal window and wish to run the program.

**Dependency Installation**

To install the necessary packages, use [pip](https://pip.pypa.io/en/stable/) and run `pip install -r requirements.txt`. This will download all of the packages found in the `requirements.txt` file.

Currently ffmpeg in its entirety is present in the binaries folder. If at some point the choice is made to remove it from the repository, instructions will be provided here on how to install it. 

In order to use custom widgets in PySide Designer, you'll need to add the "register" folder to the enviornment variable `PYSIDE_DESIGNER_PLUGINS`. If you don't have interest in gui development with designer, then don't worry about this.

Once you have done all of this, simply run `python main.py` with a terminal in the repo's directory to start the program.

# Usage

Hopefully peanut's interface is self explanatory with how to use it, but if not, its usage will be described here. First, find a playlist you want to use (from YouTube), and paste its url into the playlist url textbox, then click the "Load" button. Wait a second, and you should see a button appear with the playlist's name. Click on the button to open the audio player page of peanut. Here you will the album cover for the current track (if it is downloaded), the track list for the playlist, and various audio controls below this. The controls from left to right near the bottom of the gui are as follows: download, organize (unshuffle), previous, play, next, shuffle, and loop (for the current track only). To get started and listen to your playlist, you will first need to download it. To download the playlist, simply click the download button near the bottom of the gui. Note: peanut does support downloading and listening to a playlist at the same time, but you must wait for the current track to download before you can listen to it. The playlist downloader will stop on its own after downloading the entire playlist, but you can request it to stop by clicking on the "x" button in place of the download button. Note that this will not happen instantly, but will occur after the current track is finished downloading. If you decide to select a track that is not downloaded while the downloader is not active, that track will be skipped. If there are no more downloaded tracks left in the playlist after that point, the playlist will simply end and you will be taken to the playlist selector menu. 

Additionally, the tracks in the track list have distinct colors for each state that they have. So far, these colors are:
- Sky blue: track is currently selected and playing.
- Dark gray: track is downloaded but not playing.
- Light gray: track is not downloaded.
- Orange: track is selected but is still downloading.

The following hotkeys currently exist for peanut (with support planned for custom hotkey combos):
- Play: "alt+p",
- Skip: "alt+n",
- Previous: "alt+o",
- Loop: "alt+l",
- Shuffle: "alt+s",
- Organize: "alt+m",
- Kill program: "alt+k",

When you are finished with peanut, simply closing the window will request its closure. However, peanut will not immediately close, but will do so after all of its internal processes have finished (including the downloader). Be aware that forcibly closing peanut before this can finish may result in strange file behavior and errors the next time you open peanut. To fix this, just delete the "output" folder in peanut's main directory and reopen the program.

# Contributing

Currently, all contributors for this repository are added on an invite-only basis. If you wish to be added to the contributing team, just let me know. However, any pull requests to add features or fix bugs present in the main repository are welcome as well.

If you are working in your own fork of the repository, you can update your version of the `requirements.txt` file by simply running `pip freeze > requirements.txt`.

# Credits

Just like any project, this one has its own list of contributors, credits, and libraries that were used.

The following public libraries were used in this project:
- [Dependency-Injector](https://pypi.org/project/dependency-injector/)
- [just_playback](https://pypi.org/project/just_playback/)
- [keyboard](https://pypi.org/project/keyboard/)
- [mutagen](https://pypi.org/project/mutagen/)
- [pillow](https://pypi.org/project/pillow/)
- [PySide6](https://pypi.org/project/PySide6/)
- [requests](https://pypi.org/project/requests/)
- [yt_dlp](https://pypi.org/project/yt-dlp/)
- [ytmusicapi](https://pypi.org/project/ytmusicapi/)
- [pyacoustid](https://pypi.org/project/pyacoustid/)
- [pypresence](https://pypi.org/project/pypresence/)

The majority of the icons in this project are from https://www.flaticon.com/.

# License

This project uses the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.en.html).

# Privacy

This program does not collect any data on those who use it. The only data stored on the device that the program runs on is that which is necessary to save the playlists that are loaded, as well as any preferences. No information is sent to any remote servers except for YouTube and YouTube Music, and those requests are only made to provide functionality. 

# Disclaimer

This program is intended for personal and/or educational use only. The developers of this project do not support or endorse using this program to download content that is protected by copyright laws or is not rightfully theirs, and are not responsible for the actions of the users of this program.