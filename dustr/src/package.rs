use ::std::convert::TryFrom;

impl crate::Package {
    pub fn build(&self, dest: ::std::path::PathBuf) -> ::anyhow::Result<()> {
        if dest.exists() {
            ::std::fs::remove_dir_all(&dest)?;
        }
        ::std::fs::create_dir_all(dest.join("lib"))?;

        let pubspec = generate_pubspec(self.name.clone());
        ::std::fs::write(dest.join("pubspec.yaml"), ::serde_yaml::to_string(&pubspec)?)?;
        initialize_dependencies(&dest)?;
        for lib in &self.libraries {
            lib.build(&dest.join("lib"))?;
        }
        ::std::fs::write(dest.join("lib/dylib.dart"), crate::Library::new_dylib(&self.name))?;
        copy_dustr_lib(&dest)?;

        Ok(())
    }
}

impl crate::Package {
    pub fn new(name: String, crate_paths: Vec<::std::path::PathBuf>) -> ::anyhow::Result<Self> {
        Ok(Self{
            name: name.clone(),
            libraries: crate_paths.into_iter().map(|path| {
                crate::Library::try_from((crate::Module::from_crate(path)?, name.clone()))
            }).collect::<Result<Vec<_>, _>>()?,
        })
    }
}

fn initialize_dependencies(dest: &::std::path::PathBuf) -> ::anyhow::Result<()> {
    let output = ::std::process::Command::new("pub").args(&["get"]).current_dir(dest).output()?;
    if !output.status.success() {
        Err(::anyhow::Error::msg(format!(
            "Pub dependency initialiation failed: code={}, stdout={}, stderr={}",
            output.status,
            String::from_utf8_lossy(&output.stdout[..]),
            String::from_utf8_lossy(&output.stderr[..]),
        )))
    } else {
        Ok(())
    }
}

fn copy_dustr_lib(dest: &::std::path::PathBuf) -> ::anyhow::Result<()> {
    ::std::fs::create_dir_all(dest.join("lib/dustr"))?;
    ::std::fs::write(dest.join("lib/dustr/dustr.dart"), include_str!("../lib/dustr.dart"))?;
    ::std::fs::write(dest.join("lib/dustr/result.dart"), include_str!("../lib/result.dart"))?;
    ::std::fs::write(dest.join("lib/dustr/string.dart"), include_str!("../lib/string.dart"))?;
    Ok(())
}

fn generate_pubspec(name: String) -> Pubspec {
    let mut dependencies = ::std::collections::HashMap::new();
    dependencies.insert(
        "ffi".to_owned(),
        Dependency{version: Some("^0.1.3-dev.3".to_owned()), path: None},
    );
    Pubspec{name, dependencies, environment: Environment{sdk: ">=2.0.0 <3.0.0".to_owned()}}
}

#[derive(Debug, Serialize)]
struct Pubspec {
    name: String,
    dependencies: ::std::collections::HashMap<String, Dependency>,
    environment: Environment,
}

#[derive(Debug, Serialize)]
struct Dependency {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<::std::path::PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
}

#[derive(Debug, Serialize)]
struct Environment {
    sdk: String,
}
