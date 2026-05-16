from PIL import Image

im = Image.open("assets/cell.png").convert('RGB')

with open("assets/cell.raw", 'wb') as f:
    f.write(im.tobytes())
