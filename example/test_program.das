# Push two colors
fragPos 0 texturePixel splitColor
fragPos 1 texturePixel splitColor

# drop all but red
drop drop drop
# rotate to replace texture 0's blue
# [r g b a r2] -> [r g r2 a]
2 rotN drop

# load new color drop alpha
# [r g r2 a] -> [r g r2 a | r g b]
fragPos 1 texturePixel splitColor drop

rot drop rot drop

3 rotN
drop


makeColor