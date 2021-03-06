use crate::Header;

extern crate alloc;

pub const fn next_aligned(n: usize, alignment: usize) -> usize {
  let remaining = n % alignment;
  if remaining == 0 {
    n
  } else {
    n + (alignment - remaining)
  }
}

pub const fn next_capacity<T>(capacity: usize) -> usize {
  let elem_size = core::mem::size_of::<T>();

  if capacity == 0 {
    return match elem_size {
      1 => 8,
      2..=1024 => 4,
      _ => 1,
    };
  }

  2 * capacity
}

pub fn max_align<T>() -> usize {
  let align_t = core::mem::align_of::<T>();
  let header_align = core::mem::align_of::<Header>();
  core::cmp::max(align_t, header_align)
}

pub fn make_layout<T>(capacity: usize, alignment: usize) -> alloc::alloc::Layout {
  let header_size = core::mem::size_of::<Header>();
  let num_bytes = if capacity == 0 {
    next_aligned(header_size, alignment)
  } else {
    next_aligned(header_size, alignment)
      + next_aligned(capacity * core::mem::size_of::<T>(), alignment)
  };

  alloc::alloc::Layout::from_size_align(num_bytes, alignment).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn next_aligned_test() {
    assert_eq!(next_aligned(9, 4), 12);
    assert_eq!(next_aligned(13, 4), 16);
    assert_eq!(next_aligned(12, 4), 12);
    assert_eq!(next_aligned(13, 1), 13);
    assert_eq!(next_aligned(8, 8), 8);
    assert_eq!(next_aligned(16, 32), 32);
    assert_eq!(next_aligned(16, 512), 512);
  }

  #[repr(align(512))]
  struct OverAligned {
    _data: [u8; 512],
  }

  #[test]
  fn max_align_test() {
    let header_alignment = core::mem::align_of::<Header>();

    assert!(core::mem::align_of::<i32>() <= core::mem::align_of::<Header>());
    assert_eq!(max_align::<i32>(), header_alignment);

    assert!(core::mem::align_of::<u8>() <= core::mem::align_of::<Header>());
    assert_eq!(max_align::<u8>(), header_alignment);

    assert!(core::mem::align_of::<OverAligned>() > core::mem::align_of::<Header>());
    assert_eq!(
      max_align::<OverAligned>(),
      core::mem::align_of::<OverAligned>()
    );
  }

  #[test]
  fn make_layout_test() {
    // empty
    //
    let layout = make_layout::<i32>(0, max_align::<i32>());

    assert_eq!(layout.align(), core::mem::align_of::<Header>());
    assert_eq!(layout.size(), core::mem::size_of::<Header>());

    // non-empty, less than
    //
    let layout = make_layout::<i32>(512, max_align::<i32>());
    assert!(core::mem::align_of::<i32>() < core::mem::align_of::<Header>());
    assert_eq!(layout.align(), core::mem::align_of::<Header>());
    assert_eq!(
      layout.size(),
      core::mem::size_of::<Header>() + 512 * core::mem::size_of::<i32>()
    );

    // non-empty, equal
    //
    let layout = make_layout::<i64>(512, max_align::<i64>());
    assert_eq!(
      core::mem::align_of::<i64>(),
      core::mem::align_of::<Header>()
    );
    assert_eq!(layout.align(), core::mem::align_of::<Header>());
    assert_eq!(
      layout.size(),
      core::mem::size_of::<Header>() + 512 * core::mem::size_of::<i64>()
    );

    // non-empty, greater
    let layout = make_layout::<OverAligned>(512, max_align::<OverAligned>());
    assert!(core::mem::align_of::<OverAligned>() > core::mem::align_of::<Header>());
    assert_eq!(layout.align(), core::mem::align_of::<OverAligned>());
    assert_eq!(
      layout.size(),
      next_aligned(
        core::mem::size_of::<Header>(),
        core::mem::align_of::<OverAligned>()
      ) + 512 * core::mem::size_of::<OverAligned>()
    );

    // non-empty, over-aligned
    let layout = make_layout::<i32>(512, 32);
    assert_eq!(layout.align(), 32);
    assert_eq!(
      layout.size(),
      next_aligned(core::mem::size_of::<Header>(), 32)
        + next_aligned(core::mem::size_of::<i32>() * 512, 32)
    );
  }
}
