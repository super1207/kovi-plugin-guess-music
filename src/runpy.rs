use std::{ffi::{c_char, CStr, CString}, path::PathBuf};

use kovi::serde_json;

extern "system" {
    // // https://super1207.github.io/redreply/#/detailref/?id=%e8%bf%90%e8%a1%8c%e6%9c%ac%e5%9c%b0py
    // fn a9d0d1038bfd4e2b9543d2ef67101731_run_local_python(code: *const c_char,input: *const c_char,app_dir: *const c_char) -> *mut c_char;

    // https://super1207.github.io/redreply/#/detailref/?id=%e8%bf%90%e8%a1%8cpy
    fn a9d0d1038bfd4e2b9543d2ef67101731_run_virtual_python(code: *const c_char,input: *const c_char,app_dir: *const c_char,app_flag: *const c_char) -> *mut c_char;
    
    fn a9d0d1038bfd4e2b9543d2ef67101731_free(ptr: *mut c_char) -> *mut c_char;
}

// pub fn run_local_python(code:&str,input:&str,app_dir:PathBuf) -> Result<String, Box<dyn std::error::Error>> {
//     let code_t = CString::new(code)?;
//     let input_t = CString::new(input)?;
//     let app_dir_t2 = app_dir.to_string_lossy().to_string();
//     let app_dir_t = CString::new(app_dir_t2)?;
//     let ret = unsafe { a9d0d1038bfd4e2b9543d2ef67101731_run_local_python(code_t.as_ptr(),input_t.as_ptr(),app_dir_t.as_ptr()) };
//     let ret_str = unsafe { CStr::from_ptr(ret).to_str() }.unwrap().to_string();
//     unsafe { a9d0d1038bfd4e2b9543d2ef67101731_free(ret) };
//     let js:serde_json::Value = serde_json::from_str(&ret_str)?;
//     let retcode = &js["retcode"].as_i64().ok_or("retcode not a number")?;
//     let data = &js["data"].as_str().ok_or("data not a string")?;
//     if *retcode != 0 {
//         return Err((*data).into());
//     }
//     Ok(data.to_string())
// }

pub fn run_virtual_python(code:&str,input:&str,app_dir:PathBuf,app_flag:&str) -> Result<String, Box<dyn std::error::Error>> {
    let code_t = CString::new(code)?;
    let input_t = CString::new(input)?;
    let app_dir_t2 = app_dir.to_string_lossy().to_string();
    let app_dir_t = CString::new(app_dir_t2)?;
    let app_flag_t = CString::new(app_flag)?;
    let ret = unsafe { a9d0d1038bfd4e2b9543d2ef67101731_run_virtual_python(code_t.as_ptr(),input_t.as_ptr(),app_dir_t.as_ptr(),app_flag_t.as_ptr()) };
    let ret_str = unsafe { CStr::from_ptr(ret).to_str() }.unwrap().to_string();
    unsafe { a9d0d1038bfd4e2b9543d2ef67101731_free(ret) };
    let js:serde_json::Value = serde_json::from_str(&ret_str)?;
    let retcode = &js["retcode"].as_i64().ok_or("retcode not a number")?;
    let data = &js["data"].as_str().ok_or("data not a string")?;
    if *retcode != 0 {
        return Err((*data).into());
    }
    Ok(data.to_string())
}