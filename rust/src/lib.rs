use pyo3::prelude::*;
use pyo3::exceptions::PyRuntimeError;
use pyo3::types::{PyDict, PyBytes};
use ::twinleaf::tio::*;
use ::twinleaf::*;

#[pyclass(name = "DataIterator", subclass)]
struct PyIter {
    port: data::Device,
    n: Option<usize>,
    columns: Vec<String>,
}

#[pymethods]
impl PyIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> PyResult<Option<PyObject>> {
        let dict = PyDict::new(slf.py());
        
        if let Some(ctr) = slf.n {
            if ctr == 0 {
                // TODO: drop port
                return Ok(None);
            } else {
                slf.n = Some(ctr-1);
            }
        }

        while dict.is_empty() {
            let sample = slf.port.next();

            for column in &sample.columns {
                let name = column.desc.name.clone().into_pyobject(slf.py())?;
                let name_str: String = name.extract()?;
                if slf.columns.is_empty() || slf.columns.contains(&name_str) {
                    let time = sample.timestamp_end().into_pyobject(slf.py())?;
                    dict.set_item("time", time)?;
                    match column.value {
                        data::ColumnData::Int(x)   => {dict.set_item(name, x.into_pyobject(slf.py())?)?}
                        data::ColumnData::UInt(x)  => {dict.set_item(name, x.into_pyobject(slf.py())?)?}
                        data::ColumnData::Float(x) => {dict.set_item(name, x.into_pyobject(slf.py())?)?}
                        _ => { dict.set_item(name, "UNKNOWN".into_pyobject(slf.py())?)? }
                    };
                }
            }
        }
        
        Ok(Some(dict.into()))
    }
}

#[pyclass(name = "Device", subclass)]
struct PyDevice {
    proxy: proxy::Interface,
    route: proto::DeviceRoute,
    rpc: proxy::Port,
}

#[pymethods]
impl PyDevice {
    #[new]
    #[pyo3(signature = (root_url=None, route=None))]
    fn new(root_url: Option<String>, route: Option<String>) -> PyResult<PyDevice> {
        let root = if let Some(url) = root_url {
            url
        } else {
            "tcp://localhost".to_string()
        };
        let route = if let Some(path) = route {
            proto::DeviceRoute::from_str(&path).unwrap()
        } else {
            proto::DeviceRoute::root()
        };
        let proxy = proxy::Interface::new(&root);
        let rpc = proxy.device_rpc(route.clone()).unwrap();
        Ok(PyDevice{proxy, route, rpc})
    }

    fn rpc<'py>(&self, py: Python<'py>, name: &str, req: &[u8]) -> PyResult<Bound<'py, PyBytes>> {
        match self.rpc.raw_rpc(name, req) {
            Ok(ret) => Ok(PyBytes::new(py,&ret[..])),
            _ => Err(PyRuntimeError::new_err("RPC failed")),
        }
    }

    #[pyo3(signature = (n=1, columns=None))]
    fn samples<'py>(&self, _py: Python<'py>, n: Option<usize>, columns: Option<Vec<String>>) -> PyResult<PyIter> {
        Ok(PyIter{port: data::Device::new(self.proxy.device_full(self.route.clone()).unwrap()), n: n, columns: columns.unwrap_or_default()})
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _twinleaf(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDevice>()?;
    Ok(())
}

