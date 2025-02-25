use pyo3::prelude::*;
use pyo3::exceptions::PyRuntimeError;
use pyo3::types::{PyNone,PyBytes,PyString};
use ::twinleaf::tio::*;
use ::twinleaf::*;

/*
#[pyclass(name = "DataIterator", subclass)]
struct PySample {

}
*/

#[pyclass(name = "DataIterator", subclass)]
struct PyIter {
    port: data::Device,
    n: Option<usize>,
}

#[pymethods]
impl PyIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Vec<PyObject>> {
        // TODO: blocking/nonblocking
        if let Some(ctr) = slf.n {
            if ctr == 0 {
                // TODO: figure out if we can Err here
                // TODO: drop port
                return None;
                // Or how to return a vec or none
                //PyNone::get_bound(py);
            } else {
                slf.n = Some(ctr-1);
            }
        }
        let sample = slf.port.next();
        let mut ret = vec![];
        ret.push(sample.timestamp_end().into_py(slf.py()));
        for column in sample.columns {
            let mut col = vec![];
            col.push(match column.value {
                data::ColumnData::Int(x) => {x.to_object(slf.py())}
                data::ColumnData::UInt(x) => {x.to_object(slf.py())}
                data::ColumnData::Float(x) => {x.to_object(slf.py())}
                _ => { "UNKNOWN".to_object(slf.py()) }
                //_ => {Py::<PyNone>::from_borrowed_ptr(PyNone::get_bound(py))}
            });
            col.push(column.desc.name.to_object(slf.py()));
            ret.push(col.to_object(slf.py()));
        }
        Some(ret)
    }

    /*
    fn read_next<'py>(&mut self, py: Python<'py>, blocking: bool) -> PyResult<Vec<PyObject>> {
        // TODO: blocking/nonblocking
        if let Some(ctr) = self.n {
            if ctr == 0 {
                // TODO: figure out if we can Err here
                return Ok(vec![]);
                // Or how to return a vec or none
                //PyNone::get_bound(py);
            } else {
                self.n = Some(ctr-1);
            }
        }
        let sample = self.port.next();
        let mut ret = vec![];
        ret.push(sample.timestamp_end().into_py(py));
        for column in sample.columns {
            ret.push(match column.value {
                data::ColumnData::Int(x) => {x.to_object(py)}
                data::ColumnData::UInt(x) => {x.to_object(py)}
                data::ColumnData::Float(x) => {x.to_object(py)}
                _ => { "UNKNOWN".to_object(py) }
                //_ => {Py::<PyNone>::from_borrowed_ptr(PyNone::get_bound(py))}
            })
        }
        Ok(ret)
    }
    */
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
            Ok(ret) => Ok(PyBytes::new_bound(py,&ret[..])),
            _ => Err(PyRuntimeError::new_err("RPC failed")),
        }
    }

    fn samples<'py>(&self, py: Python<'py>, n: Option<usize>) -> PyResult<PyIter> {
        Ok(PyIter{port: data::Device::new(self.proxy.device_full(self.route.clone()).unwrap()), n: n})
    }
    /*
     *             let device = proxy.device_full(route).unwrap();
    let mut device = Device::new(device);

        let proxy = proxy::Port::new(&root, None, None);
        let (tx, rx) = proxy.port(None, route.clone(), true, false).unwrap();
        let device = util::DeviceRpc::new(&proxy, Some(route));
        println!("CREATED RPC PORT");
        Ok(PyRpc{port: proxy, dev: device, tx: tx, rx: rx})

    */
    /*
    fn read_start(&self) {
        // Just drain the rx queue
        loop {
            if let Err(x) = self.rx.try_recv() {
                println!("Drain fail: {}", x);
                break;
            }
        }
    }

    fn read_next<'py>(&self, py: Python<'py>, blocking: bool) -> PyResult<Vec<PyObject>> {
        loop {
            match self.rx.recv() {
                Ok(pkt) => {
                    if let proto::Payload::StreamData(sample) = pkt.payload {
                        return Ok(vec!(
                            sample.sample_n.into_py(py),
                            f32::from_le_bytes([sample.data[0], sample.data[1], sample.data[2], sample.data[3]]).to_object(py),
                            PyBytes::new_bound(py,&sample.data[..]).to_object(py))
                        );
                    }
                },
                Err(x) => {
                    return Err(PyRuntimeError::new_err(format!("Read failed: {}", x)));
                }
            }
        }
    }
*/
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _twinleaf(m: &Bound<'_, PyModule>) -> PyResult<()> {
    //m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<PyDevice>()?;
    //m.add_class::<PyIter>()?;
    Ok(())
}

