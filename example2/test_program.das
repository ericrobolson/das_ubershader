# Push original color and modify the R and B channels
fragPos 1 texturePixel splitColor

255 2 rotN drop
200 4 rotN drop

# For each fragment, use texture 0 if it's divisible by 4
fragPos 
    4 rot % 0 == rot
    4 rot % 0 == 

if && do
    fragPos 0 texturePixel splitColor
    2 rotN
    drop 255
    2 rotN
end

# Now make the G color present if the following is true:
fragPos
    3 rot % 0 == rot
    5 rot % 0 ==

if && do
    255 3 rotN drop
end

makeColor