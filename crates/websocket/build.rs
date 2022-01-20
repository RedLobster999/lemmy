fn main() -> Result<(), Box<dyn std::error::Error>> {
  rosetta_build::config()
    .source("en", "translations/translations/en.json")
    .source("fr", "translations/translations/fr.json")
    .fallback("en")
    .generate()?;

  Ok(())
}
