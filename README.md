### Build

```bash
python3 -m venv .venv
source ./.venv/bin/activate
pip install maturin
maturin build
```

### Install

```bash
pip install resvg_python
```

```python
import resvg_python

with open("example.svg", "r") as svg_string:
  png_image_bytes = resvg_python.svg_to_png(svg_string)
```
