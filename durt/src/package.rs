use ::std::convert::TryFrom;

impl crate::Package {
    pub fn build(&self, dest: ::std::path::PathBuf) -> ::anyhow::Result<()> {
        ::std::fs::remove_dir_all(&dest)?;
        ::std::fs::create_dir_all(dest.join("lib"))?;

        let pubspec = generate_pubspec(self.name.clone(), self.local_durt_lib.clone());
        std::fs::write(dest.join("pubspec.yaml"), ::serde_yaml::to_string(&pubspec)?)?;
        initialize_dependencies(&dest)?;
        for lib in &self.libraries {
            lib.build(&dest.join("lib"))?;
        }

        Ok(())
    }
}

impl crate::Package {
    pub fn new(
        name: String,
        local_durt_lib: Option<::std::path::PathBuf>,
        crate_paths: Vec<::std::path::PathBuf>,
    ) -> ::anyhow::Result<Self> {
        Ok(crate::Package{
            name,
            local_durt_lib,
            libraries: crate_paths.into_iter().map(|path| {
                crate::Library::try_from(crate::Module::from_crate(path)?)
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

fn generate_pubspec(name: String, local_durt_lib: Option<::std::path::PathBuf>) -> Pubspec {
    let mut dependencies = ::std::collections::HashMap::new();
    dependencies.insert("durt".to_owned(), if let Some(path) = local_durt_lib {
        Dependency{version: None, path: Some(path)}
    } else {
        Dependency{version: Some("v0.1.0".to_owned()), path: None}
    });

    Pubspec{name, dependencies, environment: Environment{sdk: ">=2.0.0 <3.0.0".to_owned()}}
}

#[derive(Debug, Serialize)]
struct Pubspec {
    name: String,
    dependencies: ::std::collections::HashMap<String, Dependency>,
    environment: Environment,
}

#[derive(Debug, Serialize)]
pub struct Dependency {
    path: Option<::std::path::PathBuf>,
    version: Option<String>,
}

#[derive(Debug, Serialize)]
struct Environment {
    sdk: String,
}
