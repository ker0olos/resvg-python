use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn svg_to_png(svg_data: &str) -> PyResult<Vec<u8>> {
    let opt = usvg::Options::default();

    let tree = usvg::Tree::from_str(svg_data, &opt)
        .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("Failed to parse SVG"))?;

    let pixmap_size = tree.size().to_int_size();

    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Failed to render SVG"))?;

    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    let png_data = pixmap
        .encode_png()
        .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("Failed to encode PNG"))?;

    Ok(png_data)
}

#[pymodule]
fn resvg_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(svg_to_png, m)?)?;
    Ok(())
}
