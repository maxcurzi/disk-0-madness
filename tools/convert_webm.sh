

ffmpeg -i $1 -filter_complex "scale=w=160:h=-1:flags=lanczos, palettegen=stats_mode=diff" mypalette.png
ffmpeg -i $1 -r 30 -f image2 tmp/image_%02d.png
ffmpeg -framerate 30 -i tmp/image_%02d.png -i mypalette.png -filter_complex "[0]scale=w=160:h=-1[x];[x][1:v] paletteuse" -pix_fmt rgb8 $2
