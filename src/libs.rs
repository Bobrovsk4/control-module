use libloading::{Library, Symbol};
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

#[derive(Debug, Clone)]
pub struct DLLAlgorithm {
    pub name: Symbol<'static, extern "C" fn() -> *const c_char>,
    pub exec: Symbol<'static, extern "C" fn(*const i32, usize, usize) -> *const c_char>,
}

impl DLLAlgorithm {
    pub fn load(path: &str) -> Self {
        unsafe {
            let lib: &'static mut Library = Box::leak(Box::new(Library::new(path).unwrap()));

            let name = lib
                .get::<extern "C" fn() -> *const c_char>(b"name")
                .unwrap();
            let exec = lib
                .get::<extern "C" fn(*const i32, usize, usize) -> *const c_char>(b"exec")
                .unwrap();

            DLLAlgorithm { name, exec }
        }
    }
    pub fn load_all(directory: &str) -> Vec<DLLAlgorithm> {
        let mut algs = Vec::new();
        for entry in std::fs::read_dir(directory).expect("Папка не найдена") {
            let path = entry.unwrap().path();
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

            if ["dll", "so", "dylib"].contains(&ext) {
                if let Some(path_str) = path.to_str() {
                    algs.push(DLLAlgorithm::load(path_str));
                }
            }
        }
        algs
    }

    pub fn run(&self, matrix: &Vec<Vec<i32>>) -> String {
        if matrix.is_empty() {
            return "Ошибка: пустая матрица".to_string();
        }

        let rows = matrix.len();
        let cols = matrix[0].len();

        if !matrix.iter().all(|r| r.len() == cols) {
            return "Ошибка: неравномерная матрица".to_string();
        }

        if rows.checked_mul(cols).is_none() {
            return "Ошибка: слишком большая матрица".to_string();
        }

        let flat: Vec<i32> = matrix.iter().flatten().copied().collect();

        eprintln!(
            "[FFI CALL] exec(data_ptr={:p}, rows={}, cols={})",
            flat.as_ptr(),
            rows,
            cols
        );

        unsafe {
            let ptr = (self.exec)(flat.as_ptr(), rows, cols);
            if ptr.is_null() {
                return "Ошибка: плагин вернул null".to_string();
            }

            let _ = CString::from_raw(ptr as *mut c_char);

            CStr::from_ptr(ptr)
                .to_str()
                .unwrap_or("<invalid utf8>")
                .to_string()
        }
    }
}

impl std::fmt::Display for DLLAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let ptr = (self.name)();
            match CStr::from_ptr(ptr).to_str() {
                Ok(s) => write!(f, "{}", s),
                Err(_) => write!(f, "<invalid name>"),
            }
        }
    }
}
