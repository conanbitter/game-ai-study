from PIL import Image
from pathlib import Path

files = ["assets/x_faded.png", "assets/cell.png", "assets/o.png", "assets/o_faded.png", "assets/selected.png", "assets/x.png"]

for filename in files:
    outfile = Path(filename).with_suffix('.raw')
    print(f"Converting \"{filename}\" -> \"{outfile}\"")

    im = Image.open(filename).convert('RGB')

    with open(outfile, 'wb') as f:
        f.write(im.tobytes())
