(*
 * Script to take successive photos of a narrative in Timemap.
 * Set `frames` to the number of steps in the narrative.
 * Change `frameRate if you want something faster/slower.
 * Open Safari with the timemap, and manually add "id='clickmepls'" to the right arrow div in narrative mode.
 * Requires `ffmpeg` to compose the frames into a video
 *)

set frames to 370
set dFolder to "~/Desktop/narrativecapture/"
set frameRate to 3

do shell script ("mkdir -p " & dFolder)

set i to 0
delay 3 -- Wait for 30 seconds.
repeat frames times
        tell application "Safari"
                activate
                set winID to id of window 1
        end tell
        do shell script ("screencapture " & dFolder & "frame-" & i & ".png")
        tell application "Safari"
                -- NOTE: you need to manually add the id 'clickmepls' to the narrative arrow in Safari.
                do JavaScript "document.getElementById('clickmepls').click();" in current tab of first window
        end tell
        delay 1.5
        set i to i + 1
end repeat

do shell script ("ffmpeg -r " & frameRate & " -s 1920x1080 -start_number 0 -i frame-%d.png -vframes " & frames & " timemap_capture.mp4")
