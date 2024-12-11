extern "C" {
  pub fn JSC__initialize();
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_initialize() {
    unsafe {
      JSC__initialize();
    }
  }
}
