//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

/*
build.rs 通过向标准输出打印特定前缀的行（println!("cargo:...")）来向 Cargo 传递指令，例如：
告诉 Cargo 链接库或添加搜索路径（如 cargo:rustc-link-search=...、cargo:rustc-link-lib=...）；
设置编译期/构建期环境变量（如 cargo:rustc-env=KEY=VALUE）；
声明重新运行条件（如 cargo:rerun-if-changed=...）。
*/

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    let your_command = format!(
        "rustc-env=TEST_FOO={}",
        timestamp
    );
    println!("cargo:{}", your_command);
    
    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    let your_command = r#"rustc-cfg=feature="pass""#;
    println!("cargo:{}", your_command);
}
