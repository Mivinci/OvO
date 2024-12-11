// extern "C" {
//   fn JSC__VM__create(heap_type: HeapType);
// }

// #[repr(C)]
// pub enum HeapType {
//   Small,
//   Large,
// }

// #[cfg(test)]
// mod test {
//   use super::*;

//   #[test]
//   fn test_initialize() {
//     unsafe {
//       JSC__VM__create(HeapType::Small);
//     }
//   }
// }
