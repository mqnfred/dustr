use ::itertools::Itertools;
use ::std::convert::TryInto;

impl crate::Library {
    pub fn build(&self, dest: &::std::path::PathBuf) -> ::anyhow::Result<()> {
        ::std::fs::write(dest.join(format!("{}.dart", self.name)), format!(
            "{imports}\n\n{structs}\n\n{enums}\n\n{functions}\n\n{wrappers}\n\n",
            imports = self.imports,
            structs = self.structs.iter().join("\n"),
            enums = self.enums.iter().join("\n"),
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

impl ::std::convert::TryFrom<(crate::Module, String)> for crate::Library {
    type Error = ::anyhow::Error;
    fn try_from((m, pkg): (crate::Module, String)) -> ::anyhow::Result<Self> {
        Ok(Self{
            name: m.name.clone(),
            imports: crate::Imports::from_module(&m, &pkg),
            structs: m.structs.iter().map(|d| crate::Struct::from_data(d)).collect::<Result<Vec<_>, _>>()?,
            enums: m.enums.iter().map(|d| d.try_into()).collect::<Result<Vec<_>, _>>()?,
            functions: m.functions.iter().map(|f| f.try_into()).collect::<Result<Vec<_>, _>>()?,
            wrappers: m.functions.iter().map(|f| f.try_into()).collect::<Result<Vec<_>, _>>()?,
            subs: m.subs.into_iter().map(|m| {
                Self::try_from((m, pkg.clone()))
            }).collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl crate::Library {
    pub fn new_dylib(pkg: &str) -> String {
        format!(
            "import 'dart:ffi';\n\
            \n\
            final dylib = DynamicLibrary.open('lib{lib_name}.so');\n",
            lib_name = pkg,
        )
    }
}
