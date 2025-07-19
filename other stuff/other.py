import yt_dlp
from yt_dlp import YoutubeDL
from concurrent.futures import ThreadPoolExecutor, as_completed

def get_playlist_video_urls(playlist_url):
    ydl_opts = {
        'extract_flat': True,  # Extract only basic information to make it faster
        'force_generic_extractor': True,
        'dump_single_json': True, # Request the data in JSON format
        'flat_playlist': True, # Extract only video IDs and titles from the playlist
    }

    with yt_dlp.YoutubeDL(ydl_opts) as ydl:
        try:
            info_dict = ydl.extract_info(playlist_url, download=False)
            video_urls = []
            if 'entries' in info_dict:
                for entry in info_dict['entries']:
                    if entry and 'url' in entry:
                        video_urls.append(entry['url'])
            return video_urls
        except Exception as e:
            print(f"Error extracting playlist info: {e}")
            return []

urls = []

if __name__ == "__main__":
    playlist_url = "https://www.youtube.com/playlist?list=PLBO2h-GzDvIbFt6A3iEs8My8MzPoSMdM5"  # Replace with your actual playlist URL
    urls = get_playlist_video_urls(playlist_url)
    
    

def extract_info(url):
    ydl_opts = {
        'quiet': False,
        'skip_download': True,
        'nocheckcertificate': True,
    }
    with YoutubeDL(ydl_opts) as ydl:
        try:
            info = ydl.extract_info(url, download=False)
            return url, info
        except Exception as e:
            return url, e

def main(urls, max_workers=1):
    results = {}
    with ThreadPoolExecutor(max_workers=max_workers) as executor:
        futures = {executor.submit(extract_info, url): url for url in urls}
        for future in as_completed(futures):
            url, result = future.result()
            results[url] = result
            if isinstance(result, Exception):
                print(f"Failed to extract {url}: {result}")
            else:
                print(f"Extracted {url}: {result.get('title', 'No title')}")

    return results

if __name__ == "__main__":
    video_urls = urls
    info_results = main(video_urls, max_workers=1)

    # Example: print the best audio URL for each video
    for url, info in info_results.items():
        if isinstance(info, Exception):
            print(f"Error with {url}: {info}")
            continue
        best_audio = None
        if 'requested_formats' in info:
            for f in info['requested_formats']:
                if f.get('acodec') != 'none':
                    best_audio = f['url']
                    break
        else:
            # fallback if no requested_formats
            for f in info.get('formats', []):
                if f.get('acodec') != 'none':
                    best_audio = f['url']
                    break
        print(f"Best audio URL for {url}:\n{best_audio}\n")
