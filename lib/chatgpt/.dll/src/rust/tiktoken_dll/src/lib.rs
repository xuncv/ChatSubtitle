use std::ffi::CStr;
use std::os::raw::c_char;
use tiktoken::SpecialTokenAction;
use tiktoken::SpecialTokenHandling;
use lazy_static::lazy_static;

lazy_static! {
    static ref ENCODING_FACTORY: tiktoken::Encoding = {
        tiktoken::EncodingFactory::cl100k_base().unwrap()
    };
}

#[no_mangle]
pub extern "C" fn gpt_num_tokens(ptr_utf8: *const c_char) -> i32 {
    // 检查指针是否为空，若为空则返回 0
    if ptr_utf8.is_null() {
        return 0;
    }

    // 将指针转为 C 字符串并尝试转换为 Rust 字符串
    let s_utf8 = match unsafe { CStr::from_ptr(ptr_utf8) }.to_str() {
        Ok(s) => s,
        Err(_) => "",
    };

    // 使用静态的 EncodingFactory 实例对字符串进行编码并返回 tokens 长度
    ENCODING_FACTORY.encode(
        s_utf8,
        &SpecialTokenHandling {
            default: SpecialTokenAction::Forbidden,
            ..Default::default()
        },
    )
    .map(|tokens| tokens.len() as i32)
    .unwrap_or(0)
}
