//! This library was created to assist with parsing endian sensitive content.
//! It allows us to parse the data normally as though it was native endianness.
//! Then when we need the value we can cast the value to the desired endianness.
//! 
//! This saves us from having to create conditional parsing using T::from_xx_bytes,
//! and instead allows us to lazily parse the values. then convert them when we need them.
//! this acts kinda like a endian "superposition".
//! 
//! Since this also wraps all the basic scalar types, it gives us a common type
//! to pass to functions that tags the data as endian sensitive. 
//! 
//! Also included are helper functions to make loading values from a stream quicker.
//! 
//! Let's look at a basic use case. We have a file that could of been created on
//! a big or little endian system. The implementation of the file specification
//! states the first byte in the file will signal if the rest of the file is
//! big or little endian. Let's say 0x00 for little endian, and 0x01 for big.
//! 
//! Let's create a program that parses the first byte and stores the endianness
//! of the file. Then using that endianness cast a value read from the file.
//! 
//! ```
//! use scalar_types::Endian;
//! use std::io::{BufReader, Result};
//!  
//! fn read_some_stuff() -> Result<()> {
//!     // Binary file contains 01 | 00 00 00 02 .. ..
//!     //      Big Endian Flag-^    ^-Big Endian 0x2
//!     // System endianness is little endian
//!     let file = std::fs::File::open("file.bin")?;
//!     let mut reader = BufReader::new(file);
//! 
//!     // File endianness changes based on system that made it
//!     // The first byte in the file determines the endianness; 
//!     // 0 = little, 1 = big
//!     let mut buffer = vec![0u8];
//!     reader.read_exact(&mut buffer)?;
//! 
//!     // We can then store the endianness as a variable
//!     // and use this to cast. Allowing us to dynamically
//!     // cast the data based on the content of the file
//!     let endianness = {
//!         if buffer[0] == 0 { 
//!             Endian::Little(()) 
//!         } else { 
//!             Endian::Big(()) 
//!         }
//!     };
//! 
//!     // Try and read the endian sensitive content normally from stream
//!     // Endian values are Endian::Native by default
//!     let endian_value = Endian::<u32>::from_stream(&mut reader);
//!     let parsed_value = match endian_value {
//!         Some(value) => value,
//!         None => panic!("Unable to parse value from stream!")
//!     };
//! 
//!     // Then we convert the value only when we need it.
//!     // Saving us from having to cast u32::from_xx_bytes for every value
//!     let expanded_value = match parsed_value.cast(endianness) {
//!         Some(value) => value,
//!         None => 0u32
//!     };
//! 
//!     assert_eq!(expanded_value, 2);
//!     Ok(())
//! }
//! ```
//! 
//! We can also work the other way around! Let's create the file we just parsed
//! by using Endian. We could easily use to_le_bytes and to_be_bytes; 
//! however, doing so would mean we need to alternate between the two depending on
//! the system we're targeting. Instead, we can just store the target endianness and use the 
//! same code. Dynamically switching the endianness with a stored variable instead.
//! This is a lot cleaner. 
//! 
//! Instead of using to_le_bytes or to_be_bytes; we'll
//! call the to_ne_bytes, and let Endian handle the converting.
//! 
//! ```
//! use scalar_types::Endian;
//! use std::io::{Result, Write};
//! use std::fs::File;
//! use std::path::Path;
//! fn write_some_stuff() -> Result<()> {
//!     // Open a file hand for our output
//!     let mut output = File::create(Path::new("file.bin"))?;
//! 
//!     // since we're a little endian system, writing a big endian file
//!     // we will assign this value explicitly.
//!     // let's store this for later to use it to cast
//!     let endianness = Endian::Big(());
//! 
//!     // Note that Endian::Native isn't actually aware of the system endianness,
//!     // it instead acts as an abstract container like a "superposition". 
//!     // If we wanted to get our native endianness "get_native_endianness()"
//!     // is a helper function that will return the correct endianness.
//!     // let endianness = get_native_endianness()?;
//! 
//!     // Moving forward:
//!     // Our spec says we need to write 0x01 for big endian, or 0x00 for little.
//!     if endianness.is_big() {
//!         write!(output, "\x01")?;
//!     } else {
//!         write!(output, "\x00")?;
//!     };
//! 
//!     // Next we're going to create our native endian 0x02 value
//!     let endian_value = Endian::new(2u32);
//! 
//!     // Finally we write the output
//!     if let Some(value) = endian_value.cast(endianness) {
//!         // We've already handled endianness, so we will use the built-in to_ne_bytes function
//!         output.write(&value.to_ne_bytes())?;
//!     }
//! 
//!     Ok(())
//! }
//! ```
//! 
//! This was created to assist me with creating modding tools for video games. As some games share the same
//! data, but the endianness changes based off the console it was built for. Working this way allows me to
//! use the same code for all systems, and cast the values to the native endianness dynamically when needed.
//! I figured I would share it on the off chance other people may find it useful.

/// This error shouldn't really be possible, but out of abundance of caution it has been included.
pub enum Error {
    UnknownArchitecture
}

/// Get's the systems native endianness
pub fn get_native_endianness() -> Result<Endian<()>, Error> {
    union SharedMemory {
        value: u16,
        array: [u8; 2]
    }
    
    unsafe {
        let endian_tester = SharedMemory { value: 0x1234 };

        match endian_tester.array[0] {
            0x34 => Ok(Endian::Little(())),
            0x12 => Ok(Endian::Big(())),
            _  => Err(Error::UnknownArchitecture)   // Shouldn't be possible, but out of abundance of caution
        }
    }
}

/// Endian
/// This wraps a scalar value and specializes the value for a specific endianness.
/// In doing so it allows us to tag endian sensitive content, and safely pass it between functions
/// for applications that may require this.
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Endian<T> {
    Little(T),
    Big(T),
    Native(T)
}

/// UNSAFE
/// Swap the endianness of a value by casting the value's memory
/// to a slice and reversing the slice.
/// 
/// Marked unsafe as it uses a raw pointer; however, 
/// the unsafe code is bounded by the size of the variable
/// and should never reach unowned memory.
fn endian_swap_unsafe<DataT>(mut value: DataT) -> DataT {
    let ptr: *mut DataT = &mut value;
    let array = unsafe { std::slice::from_raw_parts_mut(ptr as *mut u8, std::mem::size_of_val(&value)) };
    array.reverse();

    value
}


impl<T: Copy + Default> Endian<T> {
    /// All values are read in as "Endian::Native(T)". It can be converted between to the desired endianness when needed.
    /// ```
    /// use scalar_types::Endian;
    /// fn main() {
    ///     let scalar_types = Endian::new(42u16);
    /// }
    /// ```
    pub fn new(value: T) -> Endian<T> {
        Endian::Native(value)
    }

    /// UNSAFE
    /// 
    /// Reads and returns a Endian::Native(T) from any type that implements the std:io::Read trait. 
    /// Advances the stream by the size of type T bytes.
    /// 
    /// Marked unsafe as it uses a raw pointer; however, 
    /// the unsafe code is bounded by the size of the variable
    /// and should never reach unowned memory.
    /// ```
    /// use scalar_types::Endian;
    /// use std::io::{BufReader, Result};
    /// 
    /// fn read_some_stuff() -> Result<()> {
    ///     let file = std::fs::File::open("file.bin")?;
    ///     let mut reader = BufReader::new(file);
    /// 
    ///     let endian_value = Endian::<u32>::from_stream(&mut reader);
    ///     let parsed_value = match endian_value {
    ///         Some(value) => value,
    ///         None => panic!("Unable to parse value from stream!")
    ///     };
    ///     Ok(())
    ///  }
    /// ```
    pub fn from_stream<StreamT: std::io::Read>(stream: &mut StreamT) -> Option<Endian<T>> {
        let mut value = T::default();
        let ptr: *mut T = &mut value;
        let buffer = unsafe { std::slice::from_raw_parts_mut(ptr as *mut u8, std::mem::size_of_val(&value)) };

        match stream.read_exact(buffer) {
            Err(_) => None,
            Ok(()) => Some(Endian::Native(value))
        }
    }

    /// Attempts to cast the value held by Endian to a big endian value.
    /// Only fail condition is if get_native_endianness fails somehow
    ///
    /// This shouldn't really be possible; however, it does call unsafe. 
    /// so, out of abundance of caution we include the fail condition.
    /// ```
    /// use scalar_types::Endian;
    /// fn main() {
    ///     let scalar_types = Endian::new(42u16);
    /// 
    ///     let be_scalar_types = match scalar_types.as_big() {
    ///         Some(value) => value,
    ///         None => panic!("Unable to convert endianness!")
    ///     };
    /// }
    /// ```
    pub fn as_big(&self) -> Option<T> {
        match self {
            Endian::Little(value) => Some(endian_swap_unsafe(*value)),
            Endian::Big(value) => Some(*value),
            Endian::Native(value) => {
                match get_native_endianness() {
                    Err(_) => None,
                    Ok(order) => match order {
                        Endian::Little(()) => Some(endian_swap_unsafe(*value)),
                        Endian::Big(()) => Some(*value),

                        // Native Endianness being "Native" infinitely recursive
                        Endian::Native(()) => None 
                    }
                }
            }
        }
    }

    /// Attempts to cast the value held by Endian to a little endian value.
    /// Only fail condition is if get_native_endianness fails somehow
    ///
    /// This shouldn't really be possible; however, it does call unsafe. 
    /// so, out of abundance of caution we include the fail condition.
    /// ```
    /// use scalar_types::Endian;
    /// fn main() {
    ///     let scalar_types = Endian::new(42u16);
    /// 
    ///     let le_scalar_types = match scalar_types.as_little() {
    ///         Some(value) => value,
    ///         None => panic!("Unable to convert endianness!")
    ///     };
    /// }
    /// ```
    pub fn as_little(&self) -> Option<T>  {
        match self {
            Endian::Little(value) => Some(*value),
            Endian::Big(value) => Some(endian_swap_unsafe(*value)),
            Endian::Native(value) => {
                match get_native_endianness() {
                    Err(_) => None,
                    Ok(order) => match order {
                        Endian::Little(()) => Some(*value),
                        Endian::Big(()) => Some(endian_swap_unsafe(*value)),
                        
                        // Native Endianness being "Native" infinitely recursive
                        Endian::Native(()) => None 
                    }
                }
            }
        }
    }

    /// Attempts to cast the value held by Endian to a native endian value.
    /// Only fail condition is if get_native_endianness fails somehow
    ///
    /// This shouldn't really be possible; however, it does call unsafe. 
    /// so, out of abundance of caution we include the fail condition.
    /// ```
    /// use scalar_types::Endian;
    /// fn main() {
    ///     let scalar_types = Endian::new(42u16);
    /// 
    ///     let le_scalar_types = match scalar_types.as_native() {
    ///         Some(value) => value,
    ///         None => panic!("Unable to convert endianness!")
    ///     };
    /// }
    /// ```
    pub fn as_native(&self) -> Option<T>  {
        match self {
            Endian::Little(value) => match get_native_endianness() {
                Err(_) => None,
                Ok(order) => match order {
                    Endian::Little(()) => Some(*value),
                    Endian::Big(()) => Some(endian_swap_unsafe(*value)),
                    
                    // Native Endianness being "Native" infinitely recursive
                    Endian::Native(()) => None 
                }
            }
            
            Endian::Big(value) => {
                match get_native_endianness() {
                    Err(_) => None,
                    Ok(order) => match order {
                        Endian::Little(()) => Some(endian_swap_unsafe(*value)),
                        Endian::Big(()) => Some(*value),
                        
                        // Native Endianness being "Native" infinitely recursive
                        Endian::Native(()) => None 
                    }
                }
            }

            Endian::Native(value) => Some(*value),
        }
    }

    /// Attempts to cast the value held by Endian to a specified endianness
    /// Only fail condition is if get_native_endianness fails somehow
    ///
    /// This shouldn't really be possible; however, it does call unsafe. 
    /// so, out of abundance of caution we include the fail condition.
    /// ```
    /// use scalar_types::Endian;
    /// fn main() {
    ///     let scalar_types = Endian::new(42u16);
    /// 
    ///     let le_scalar_types = scalar_types.cast(Endian::Little(()));
    ///     let be_scalar_types = scalar_types.cast(Endian::Big(()));
    ///     let ne_scalar_types = scalar_types.cast(Endian::Native(()));
    /// }
    /// ```
    pub fn cast(&self, order: &Endian<()>) -> Option<T> {
        match order {
            Endian::Little(()) => self.as_little(),
            Endian::Big(()) => self.as_big(),
            Endian::Native(()) => self.as_native()
        }
    }

    // Returns true if Endian is a Endian::Little option
    /// ```
    /// use scalar_types::Endian;
    /// fn main() {
    ///     let scalar_types = Endian::Little(42u16);
    ///     
    ///     assert_eq!(scalar_types.is_little(), true)
    /// }
    /// ```
    pub fn is_little(&self) -> bool {
        match self {
            Endian::Little(_) => true,
            Endian::Big(_) => false,
            Endian::Native(_) => false
        }
    }

    // Returns true if Endian is a Endian::Big option
    /// ```
    /// use scalar_types::Endian;
    /// fn main() {
    ///     let scalar_types = Endian::Big(42u16);
    ///     
    ///     assert_eq!(scalar_types.is_big(), true)
    /// }
    /// ```
    pub fn is_big(&self) -> bool {
        match self {
            Endian::Little(_) => false,
            Endian::Big(_) => true,
            Endian::Native(_) => false
        }
    }

    // Returns true if Endian is a Endian::Native option
    /// ```
    /// use scalar_types::Endian;
    /// fn main() {
    ///     // new() creates a Endian::Native
    ///     let scalar_types = Endian::new(42u16);
    ///     let ne_scalar_types = Endian::Native(42u16);
    /// 
    ///     assert_eq!(scalar_types.is_native(), true);
    ///     assert_eq!(ne_scalar_types.is_native(), true);
    /// }
    /// ```
    pub fn is_native(&self) -> bool {
        match self {
            Endian::Little(_) => false,
            Endian::Big(_) => false,
            Endian::Native(_) => true
        }
    }

    /// Unpack the value as a native endian value.
    /// If casting fails, the default value for the type is returned instead
    /// Not recommended for production.
    /// ```
    /// use scalar_types::Endian;
    /// fn main() {
    ///     // new() creates a Endian::Native
    ///     let scalar_types = Endian::new(42u16);
    ///     
    ///     println!("the meaning of life the universe and everything: {}", scalar_types.unpack());
    /// }
    /// ```
    /// Output: 
    /// ```
    ///     "the meaning of life the universe and everything: 42"
    /// ```
    pub fn unpack(&self) -> T {    
        if let Some(value) = self.as_native() {
            return value;
        }
        T::default()
    }
}
