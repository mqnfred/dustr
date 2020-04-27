use ::itertools::Itertools;
use ::std::convert::TryInto;

impl crate::Library {
    pub fn build(&self, dest: &::std::path::PathBuf) -> ::anyhow::Result<()> {
        ::std::fs::write(dest.join(format!("{}.dart", self.name)), format!(
            "{imports}\n\n{functions}\n\n{wrappers}\n\n",
            imports = self.imports,
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
    fn try_from((pkg, m): (String, crate::Module)) -> ::anyhow::Result<Self> {
        Ok(Self{
            name: m.name.clone(),
            imports: crate::Imports::from_module(&pkg, &m),
            functions: m.functions.iter().map(|f| f.try_into()).collect::<Result<Vec<_>, _>>()?,
            wrappers: m.functions.iter().map(|f| f.try_into()).collect::<Result<Vec<_>, _>>()?,
            subs: m.subs.into_iter().map(|m| {
                Self::try_from((pkg.clone(), m))
            }).collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl crate::Library {
    pub fn new_dylib(pkg: &str) -> String {
        // TODO: the free_result function should probably not be declared here... but it's
        // impossible to declare within the durt/result.dart library since it does not have access
        // to dylib.
        format!(
            "import 'dart:ffi';\n\
            import 'package:durt/result.dart';\n\
            \n\
            final dylib = DynamicLibrary.open('lib{lib_name}.so');\n\
            \n\
            final void Function(Pointer<Result>) freeResult = \
            dylib.lookup<NativeFunction<\
                Void Function(Pointer<Result>)>\
            >('free_result').asFunction();\n",
            lib_name = pkg,
        )
    }
}
