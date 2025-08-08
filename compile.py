import subprocess
import sys

versionNumber = input("Specify a version number for the compiler:\n>> ")
fileDescription = "Music player that works off youtube playlists"
productName = "peanut"
companyName = "Ugly Mango Studios"

fileName = "main.py"
binaryFolderName = "binaries"

args = ["python", "-m", "nuitka", f"--include-data-dir={binaryFolderName}={binaryFolderName}", "--enable-plugin=pyside6", "--standalone", "--assume-yes-for-downloads", 
        "--lto=yes", "--include-module=_cffi_backend", "--nofollow-import-to=yt_dlp.extractor", "--include-module=encodings",
        f"--product-name={productName}", f"--file-version={versionNumber}", f"--product-version={versionNumber}", f"--file-description={fileDescription}", fileName]

result = subprocess.run(executable=sys.executable, args=args)

print("Compiling complete.")