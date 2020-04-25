use ::itertools::Itertools;
use ::std::convert::TryInto;

impl crate::Library {
    pub fn build(&self, dest: &::std::path::PathBuf) -> ::anyhow::Result<()> {
        ::std::fs::write(dest.join(format!("{}.dart", self.name)), format!(
            "{imports}\n\n{functions}\n\n{wrappers}\n\n",
            imports = self.imports.iter().join("\n"),
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

impl ::std::convert::TryFrom<crate::Module> for crate::Library {
    type Error = ::anyhow::Error;
    fn try_from(m: crate::Module) -> ::anyhow::Result<Self> {
        Ok(Self{
            name: m.name,
            imports: vec![],
            functions: m.functions.iter().map(|f| f.try_into()).collect::<Result<Vec<_>, _>>()?,
            wrappers: m.functions.iter().map(|f| f.try_into()).collect::<Result<Vec<_>, _>>()?,
            subs: m.subs.into_iter().map(|m| Self::try_from(m)).collect::<Result<Vec<_>, _>>()?,
        })
    }
}
