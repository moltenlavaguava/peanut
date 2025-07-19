import yt_dlp

def download_youtube_video_yt_dlp(video_url, save_path='./downloads'):
    ydl_opts = {
        'format': 'bestvideo+bestaudio/best',  # Download best quality video and audio
        'outtmpl': f'{save_path}/%(title)s.%(ext)s',  # Output file name template
        'merge_output_format': 'mp4',  # Merge streams into mp4
    }
    try:
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            ydl.download([video_url])
        print("Download complete!")
    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    video_link = input("Enter the YouTube video URL: ")
    download_youtube_video_yt_dlp(video_link)
