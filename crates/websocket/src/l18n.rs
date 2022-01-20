rosetta_i18n::include_translations!();

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_l10n() {
    let lang = "en";
    assert_eq!(lang.cross_posted_from(), "cross-posted from: ");
    assert_eq!(Lang::Fr.number_of_posts("2"), "2 Post");
  }
}
