use std::ffi::CStr;
use std::os::raw::c_char;
use tiktoken::SpecialTokenAction;
use tiktoken::SpecialTokenHandling;

#[no_mangle]
pub extern fn gpt_num_tokens(ptr_utf8: *const c_char)-> i32 {
    //指针转为C字符串
	let cstr_utf8 = unsafe { 
		assert!(!ptr_utf8.is_null());
		CStr::from_ptr(ptr_utf8) 
	};
    let s_utf8 = match cstr_utf8.to_str() {
		Ok(s) => s,
		Err(_) => "",
	};
    let enc = tiktoken::EncodingFactory::cl100k_base().unwrap();
    let tokens = enc.encode(
        s_utf8,
        &SpecialTokenHandling {
            default: SpecialTokenAction::Forbidden,
            ..Default::default()
        }
    ).unwrap();
    return tokens.len() as i32;
}