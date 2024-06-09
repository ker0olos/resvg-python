### Build

```bash
python3 -m venv .venv
source ./.venv/bin/activate
pip install maturin
maturin build
```

### Install

```bash
pip install https://github.com/ker0olos/resvg-python/raw/main/target/wheels/resvg_python-0.1.0-cp312-cp312-linux_x86_64.whl
```

```python
import resvg_python

with open("example.svg", "r") as svg_string:
  png_image_bytes = resvg_python.svg_to_png(svg_string)
```
