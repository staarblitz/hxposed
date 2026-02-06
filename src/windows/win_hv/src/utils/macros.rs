use core::fmt;
use crate::win::DbgPrint;

#[macro_export]
macro_rules! as_pvoid {
    ($t:ident) => {
        $t.as_mut() as *mut _ as *mut c_void
    };
}

#[macro_export]
macro_rules! get_data {
    ($k:ident, $t:ident) => {
        ($k.as_mut() as *mut _ as *mut c_void).byte_offset($k.DataOffset as _) as *mut $t
    };
}

// Taken from windows-drivers-rs
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
      ($crate::utils::macros::_print(format_args!($($arg)*)))
    };
}

#[macro_export]
macro_rules! println {
    () => {
      ($crate::print!("\n"));
    };

    ($($arg:tt)*) => {
      ($crate::print!("{}\n", format_args!($($arg)*)))
    };
}

const DBG_PRINT_MAX_TXN_SIZE: usize = 512;
pub struct DbgPrintBufWriter {
    buffer: [u8; DBG_PRINT_MAX_TXN_SIZE],
    used: usize,
}

impl fmt::Write for DbgPrintBufWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut str_byte_slice = s.as_bytes();
        let mut remaining_buffer = &mut self.buffer[self.used..Self::USABLE_BUFFER_SIZE];
        let mut remaining_buffer_len = remaining_buffer.len();

        str_byte_slice = advance_slice_to_next_non_null_byte(str_byte_slice);

        while !str_byte_slice.is_empty() {
            // Get size of next chunk of string to write and copy to buffer.
            // Chunk is bounded by either the first null byte or the remaining buffer size.
            let chunk_size = str_byte_slice
                .iter()
                .take(remaining_buffer_len)
                .take_while(|c| **c != b'\0')
                .count();
            remaining_buffer[..chunk_size].copy_from_slice(&str_byte_slice[..chunk_size]);
            str_byte_slice = &str_byte_slice[chunk_size..];

            str_byte_slice = advance_slice_to_next_non_null_byte(str_byte_slice);
            self.used += chunk_size;

            // Flush buffer if full, otherwise update amount used
            if chunk_size == remaining_buffer_len && !str_byte_slice.is_empty() {
                self.flush();
            }

            remaining_buffer = &mut self.buffer[self.used..Self::USABLE_BUFFER_SIZE];
            remaining_buffer_len = remaining_buffer.len();
        }
        Ok(())
    }
}

fn advance_slice_to_next_non_null_byte(slice: &[u8]) -> &[u8] {
    slice
        .iter()
        .position(|&b| b != b'\0')
        .map_or_else(|| &slice[slice.len()..], |pos| &slice[pos..])
}

impl DbgPrintBufWriter {
    const USABLE_BUFFER_SIZE: usize = DBG_PRINT_MAX_TXN_SIZE - 1;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn flush(&mut self) {
        // Escape if the buffer is empty
        if self.used == 0 {
            return;
        }

        self.buffer[self.used] = 0;

        unsafe {
            DbgPrint(
                c"%s".as_ptr().cast(),
                self.buffer.as_ptr().cast::<u16>(),
            );
        }

        self.used = 0;
    }
}


impl Default for DbgPrintBufWriter {
    fn default() -> Self {
        Self {
            // buffer is initialized to all null
            buffer: [0; DBG_PRINT_MAX_TXN_SIZE],
            used: 0,
        }
    }
}

pub fn _print(args: fmt::Arguments) {
    let mut buffered_writer = DbgPrintBufWriter::new();

    if fmt::write(&mut buffered_writer, args).is_ok() {
        buffered_writer.flush();
    } else {
        unreachable!("DbgPrintBufWriter should never fail to write");
    }
}