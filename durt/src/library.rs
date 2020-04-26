use ::itertools::Itertools;
use ::std::convert::TryInto;

impl crate::Library {
    pub fn build(&self, dest: &::std::path::PathBuf) -> ::anyhow::Result<()> {
        ::std::fs::write(dest.join(format!("{}.dart", self.name)), format!(
            "{imports}\n\n{functions}\n\n{wrappers}\n\n",
            imports = self.imports.iter().map(|i| format!("import '{}';", i)).join("\n"),
            functions = self.functions.iter().join("\n"),
            wrappers = self.wrappers.iter().join("\n"),
        ))?;

        if !self.subs.is_empty() {
            let subfolder = dest.join(&self.name);
            ::std::fs::create_dir(&subfolder)?;
            for sub in &self.subs {
                sub.build(&subfolder)?;
            }
        }

        Ok(())
    }
}

impl ::std::convert::TryFrom<(String, crate::Module)> for crate::Library {
    type Error = ::anyhow::Error;
    fn try_from((package_name, m): (String, crate::Module)) -> ::anyhow::Result<Self> {
        Ok(Self{
            name: m.name,
            imports: vec![
                "dart:ffi".to_owned(),
                format!("package:{}/dylib.dart", &package_name),
            ],
            functions: m.functions.iter().map(|f| f.try_into()).collect::<Result<Vec<_>, _>>()?,
            wrappers: vec![],
            //wrappers: m.functions.iter().map(|f| f.try_into()).collect::<Result<Vec<_>, _>>()?,
            subs: m.subs.into_iter().map(|m| {
                Self::try_from((package_name.clone(), m))
            }).collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl crate::Library {
    pub fn new_dylib(package_name: &str) -> String {
        format!(
            "import 'dart:ffi';\nfinal dylib = DynamicLibrary.open('lib{lib_name}.so');\n",
            lib_name = package_name,
        )
    }
}
