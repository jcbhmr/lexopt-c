#[test]
fn hello_c() -> Result<(), Box<dyn std::error::Error>> {
    std::process::Command::new("cc")
        .args(&[
            "-I",
            std::path::absolute(std::path::Path::new("target/include"))?
                .to_str()
                .ok_or("not utf-8")?,
            "-L",
            std::path::absolute(std::path::Path::new("target/debug"))?
                .to_str()
                .ok_or("not utf-8")?,
            "-l",
            "lexopt_c",
            "-o",
            std::path::absolute(std::path::Path::new("target/test/hello"))?
                .to_str()
                .ok_or("not utf-8")?,
            std::path::absolute(std::path::Path::new("tests/hello.c"))?
                .to_str()
                .ok_or("not utf-8")?,
        ])
        .status()?;
    Ok(())
}
