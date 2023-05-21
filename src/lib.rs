//! # List Todos
//! List todos, optionally with pagination
//!
//! ## Example
//!
//! ```python
//! import todors
//!
//! todors.list_todos()
//! todors.list_todos(pagination={"offset": 5})
//! todors.list_todos(pagination={"limit": 5})
//! todors.list_todos(pagination={"offset": 5, "limit": 5})
//! todors.list_todos("sqlite:///tmp/todors.db")
//! ```

#![allow(dead_code, unused_variables, unused_imports)]

use crate::traits::Controller;
use errors::TodoErrors;

use pyo3::prelude::*;

mod apidoc;
mod cli;
mod consts;
mod db;
mod entities;
mod errors;
mod grpc;
mod http;
mod logging;
mod models;
mod serializers;
mod settings;
mod traits;

impl std::convert::From<TodoErrors> for PyErr {
    fn from(err: TodoErrors) -> PyErr {
        pyo3::exceptions::PyException::new_err(err.to_string())
    }
}

impl pyo3::IntoPy<PyObject> for entities::TodoRead {
    fn into_py(self, py: Python<'_>) -> PyObject {
        use pyo3::types::PyDict;

        let todo = PyDict::new(py);
        todo.set_item("id", self.id).unwrap();
        todo.set_item("title", self.title).unwrap();
        todo.set_item("done", self.done).unwrap();

        todo.into()
    }
}

impl<'a> pyo3::FromPyObject<'a> for entities::ListRequest {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let limit = ob.get_item("limit").ok();
        let offset = ob.get_item("offset").ok();

        Ok(Self {
            limit: limit.map(|l| l.extract().unwrap_or_default()),
            offset: offset.map(|o| o.extract().unwrap_or_default()),
        })
    }
}

static mut DB_POOL: Option<db::Pool> = None;

async unsafe fn get_pool(db_url: &str) -> Result<db::Pool, TodoErrors> {
    match &DB_POOL {
        Some(pool) => Ok(pool.clone()),
        None => {
            let pool = db::connect(db_url, None).await?;
            DB_POOL = Some(pool.clone());
            Ok(pool)
        }
    }
}

fn get_controller(pool: db::Pool, settings: &settings::Settings) -> models::TodoController {
    models::TodoController::new(
        pool,
        Some(settings.pagination_limit),
        Some(settings.pagination_hard_limit),
        Some(settings.create_batch_hard_limit),
    )
}

#[pyfunction]
fn list_todos(
    py: Python<'_>,
    db_url: Option<String>,
    pagination: Option<entities::ListRequest>,
) -> PyResult<&PyAny> {
    let settings = settings::Settings::new().unwrap();
    let db_url = db_url.unwrap_or(settings.db_url.clone());

    pyo3_asyncio::tokio::future_into_py(py, async move {
        let pool = unsafe { get_pool(&db_url).await? };
        let controller = get_controller(pool, &settings);
        let todos = controller.list(pagination.unwrap_or_default()).await?;
        Ok(Python::with_gil(|py| {
            todos
                .data
                .into_iter()
                .map(|todo| todo.into_py(py))
                .collect::<Vec<_>>()
        }))
    })
}

#[pymodule]
fn todors(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(list_todos, m)?)?;
    Ok(())
}
