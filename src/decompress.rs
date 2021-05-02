use std::convert::TryInto as _;
use crate::{Image, sys};
use crate::common::{Subsamp, Colorspace, Result, Error, get_error};

/// Decompresses JPEG data into raw pixels.
#[derive(Debug)]
pub struct Decompressor {
    handle: sys::tjhandle,
}

unsafe impl Send for Decompressor {}

/// JPEG header that describes the compressed image.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DecompressHeader {
    /// Width of the image in pixels (number of columns).
    pub width: usize,
    /// Height of the image in pixels (number of rows).
    pub height: usize,
    /// Chrominance subsampling that is used in the compressed image.
    pub subsamp: Subsamp,
    /// Colorspace of the compressed image.
    pub colorspace: Colorspace,
}

impl Decompressor {
    /// Create a new decompressor instance.
    #[doc(alias = "tjInitDecompress")]
    pub fn new() -> Result<Decompressor> {
        unsafe {
            let handle = sys::tjInitDecompress();
            if !handle.is_null() {
                Ok(Decompressor { handle })
            } else {
                Err(get_error(handle))
            }
        }
    }

    /// Read the JPEG header without decompressing the image.
    pub fn read_header(&mut self, jpeg_data: &[u8]) -> Result<DecompressHeader> {
        let jpeg_data_len = jpeg_data.len().try_into()
            .map_err(|_| Error::IntegerOverflow("jpeg_data.len()"))?;
        let mut width = 0;
        let mut height = 0;
        let mut subsamp = 0;
        let mut colorspace = 0;
        let res = unsafe {
            sys::tjDecompressHeader3(
                self.handle,
                jpeg_data.as_ptr(), jpeg_data_len,
                &mut width, &mut height, &mut subsamp, &mut colorspace,
            )
        };

        if res == 0 {
            let width = width.try_into().map_err(|_| Error::IntegerOverflow("width"))?;
            let height = height.try_into().map_err(|_| Error::IntegerOverflow("height"))?;
            let subsamp = Subsamp::from_u32(subsamp as u32)?;
            let colorspace = Colorspace::from_u32(colorspace as u32)?;
            Ok(DecompressHeader { width, height, subsamp, colorspace })
        } else {
            Err(unsafe { get_error(self.handle) })
        }
    }

    /// Decompress a JPEG image in `jpeg_data` into `image`.
    ///
    /// The decompressed image is stored in the pixel data of the given `image`, which must be
    /// fully initialized by the caller. Use [`read_header`](Decompressor::read_header) to
    /// determine the image size before calling this method.
    #[doc(alias = "tjDecompress2")]
    pub fn decompress_to_slice(&mut self, jpeg_data: &[u8], image: Image<&mut [u8]>) -> Result<()> {
        image.assert_valid(image.pixels.len());

        let Image { pixels, width, pitch, height, format } = image;
        let width = width.try_into().map_err(|_| Error::IntegerOverflow("width"))?;
        let pitch = pitch.try_into().map_err(|_| Error::IntegerOverflow("pitch"))?;
        let height = height.try_into().map_err(|_| Error::IntegerOverflow("height"))?;
        let jpeg_data_len = jpeg_data.len().try_into()
            .map_err(|_| Error::IntegerOverflow("jpeg_data.len()"))?;

        let res = unsafe {
            sys::tjDecompress2(
                self.handle,
                jpeg_data.as_ptr(), jpeg_data_len,
                pixels.as_mut_ptr(), width, pitch, height, format as i32,
                0,
            )
        };

        if res == 0 {
            Ok(())
        } else {
            Err(unsafe { get_error(self.handle) })
        }
    }
}
