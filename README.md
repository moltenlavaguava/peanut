# peanut 🥜

Peanut is a youtube-based video archiver and streamer. It supports the download of most youtube videos, with support planned for other online services. 

Peanut itself is developed with Python, and has an interactive GUI.

# Features

The main features of peanut currently are:
- Video downloading with conversion to audio (.ogg) files
- Offline audio playback with playlist controls
- User friendly GUI
- System wide hotkeys to control playback without interacting with the GUI
- Artist and Album data for most youtube videos via Youtube Music

## Installation

**From Release**

Currently no releases exist. Stay tuned! :)

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

# Usage

*From Release*

To run the program from a release, locate and run the `main.exe` file located in the zip.

*From Source*

To run the program from source, first ensure you have all of the dependencies installed. Then, run `python main.py`.

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

The majority of the icons in this project are from https://www.flaticon.com/.

# License

This project uses the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.en.html).