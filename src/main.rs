mod boot;
mod catalogue;
mod catalogue_bytes;
mod constants;
mod file_descriptor;
mod run;

fn main() -> anyhow::Result<()> {
    crate::run::run()
}
