use std::convert::TryInto as _;
use crate::{Colorspace, Image, YuvImage, YuvPlanesImage, raw};
use crate::buf::{OwnedBuf, OutputBuf};
use crate::common::{Subsamp, Result, Error};
use crate::handle::Handle;

/// Compresses raw pixel data into JPEG.
#[derive(Debug)]
#[doc(alias = "tjhandle")]
pub struct Compressor {
    handle: Handle,
    subsamp: Subsamp,
}

static DEFAULT_QUALITY: i32 = 95;
static DEFAULT_SUBSAMP: Subsamp = Subsamp::None;

unsafe impl Send for Compressor {}

impl Compressor {
    /// Create a new compressor instance.
    #[doc(alias = "tj3Init")]
    pub fn new() -> Result<Compressor> {
        let mut handle = Handle::new(raw::TJINIT_TJINIT_COMPRESS)?;
        handle.set(raw::TJPARAM_TJPARAM_QUALITY, DEFAULT_QUALITY as libc::c_int)?;
        handle.set(raw::TJPARAM_TJPARAM_SUBSAMP, DEFAULT_SUBSAMP as i32 as libc::c_int)?;
        Ok(Compressor { handle, subsamp: DEFAULT_SUBSAMP })
    }

    /// Set the quality of the compressed JPEG images.
    ///
    /// The quality ranges from 1 (worst) to 100 (best).
    ///
    /// # Examples
    ///
    /// ```
    /// let mut compressor = turbojpeg::Compressor::new()?;
    /// compressor.set_quality(95)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// An error is returned if `quality` is invalid:
    ///
    /// ```
    /// let mut compressor = turbojpeg::Compressor::new()?;
    /// assert!(compressor.set_quality(120).is_err());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[doc(alias = "TJPARAM_QUALITY")]
    pub fn set_quality(&mut self, quality: i32) -> Result<()> {
        self.handle.set(raw::TJPARAM_TJPARAM_QUALITY, quality as libc::c_int)
    }

    /// Enable/disable lossless compression.
    ///
    /// # Example
    ///
    /// ```
    /// let original_image = turbojpeg::Image::mandelbrot(250, 330, turbojpeg::PixelFormat::RGB);
    ///
    /// // compress the image with lossless compression
    /// let mut compressor = turbojpeg::Compressor::new()?;
    /// compressor.set_lossless(true)?;
    /// let jpeg_data = compressor.compress_to_vec(original_image.as_deref())?;
    ///
    /// // the decompressed image is exactly the same as the original image
    /// let decompressed_image = turbojpeg::decompress(&jpeg_data, turbojpeg::PixelFormat::RGB)?;
    /// # assert_eq!(decompressed_image.width, 250);
    /// # assert_eq!(decompressed_image.height, 330);
    /// # assert_eq!(decompressed_image.format, turbojpeg::PixelFormat::RGB);
    /// for y in 0..330 {
    ///     let original_pixels = &original_image.pixels[y * original_image.pitch..][..750];
    ///     let decompressed_pixels = &decompressed_image.pixels[y * decompressed_image.pitch..][..750];
    ///     assert_eq!(original_pixels, decompressed_pixels);
    /// }
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[doc(alias = "TJPARAM_LOSSLESS")]
    pub fn set_lossless(&mut self, enabled: bool) -> Result<()> {
        self.handle
            .set(raw::TJPARAM_TJPARAM_LOSSLESS, enabled as libc::c_int)
    }

    /// Set the level of chrominance subsampling of the compressed JPEG images.
    ///
    /// Chrominance subsampling can reduce the compressed image size without noticeable loss of
    /// quality (see [`Subsamp`] for more).
    ///
    /// When compressing YUV images, the level of chrominance subsampling is set from the image,
    /// and the value set by this method is overriden.
    ///
    /// # Example
    ///
    /// ```
    /// let image = turbojpeg::Image::mandelbrot(300, 300, turbojpeg::PixelFormat::RGB);
    ///
    /// // compress the image with 2x2 chrominance subsampling
    /// let mut compressor = turbojpeg::Compressor::new()?;
    /// compressor.set_subsamp(turbojpeg::Subsamp::Sub2x2)?;
    /// let jpeg_data = compressor.compress_to_vec(image.as_deref())?;
    ///
    /// // the compressed image has the 2x2 chrominance subsampling
    /// let header = turbojpeg::read_header(&jpeg_data)?;
    /// assert_eq!(header.subsamp, turbojpeg::Subsamp::Sub2x2);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[doc(alias = "TJPARAM_SUBSAMP")]
    pub fn set_subsamp(&mut self, subsamp: Subsamp) -> Result<()> {
        self.handle.set(raw::TJPARAM_TJPARAM_SUBSAMP, subsamp as i32 as libc::c_int)
    }

    /// Set the colorspace of the compressed JPEG images.
    ///
    /// Usually, this is useful where you want to set the color space to match the pixel format of
    /// the input image, to avoid incurring losses in fidelity due to color space conversion.
    ///
    /// By default, multichannel images are converted to YCbCr. Not all pixel formats can be
    /// converted into all colorspaces: see the [Colorspace] documentation for details.
    ///
    /// # Example
    ///
    /// ```
    /// let image = turbojpeg::Image::mandelbrot(300, 300, turbojpeg::PixelFormat::RGB);
    ///
    /// // compress the image with RGB colorspace
    /// let mut compressor = turbojpeg::Compressor::new()?;
    /// compressor.set_colorspace(turbojpeg::Colorspace::RGB)?;
    /// let jpeg_data = compressor.compress_to_vec(image.as_deref())?;
    ///
    /// // the compressed image has the RGB colorspace
    /// let header = turbojpeg::read_header(&jpeg_data)?;
    /// assert_eq!(header.colorspace, turbojpeg::Colorspace::RGB);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[doc(alias = "TJPARAM_COLORSPACE")]
    pub fn set_colorspace(&mut self, colorspace: Colorspace) -> Result<()> {
        self.handle.set(raw::TJPARAM_TJPARAM_COLORSPACE, colorspace as i32 as libc::c_int)
    }

    /// Enable/disable optimized baseline entropy coding.
    ///
    /// When enabled, optimal Huffman tables will be computed for the JPEG image. Optimized
    /// baseline entropy coding will improve compression slightly (generally 5% or less), but it
    /// will reduce compression performance considerably.
    ///
    /// # Example
    ///
    /// ```
    /// let image = turbojpeg::Image::mandelbrot(500, 500, turbojpeg::PixelFormat::RGB);
    /// let mut compressor = turbojpeg::Compressor::new()?;
    ///
    /// compressor.set_optimize(false)?;
    /// let unoptimized = compressor.compress_to_vec(image.as_deref())?;
    ///
    /// compressor.set_optimize(true)?;
    /// let optimized = compressor.compress_to_vec(image.as_deref())?;
    ///
    /// assert!(optimized.len() < unoptimized.len());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[doc(alias = "TJPARAM_OPTIMIZE")]
    pub fn set_optimize(&mut self, optimize: bool) -> Result<()> {
        self.handle.set(raw::TJPARAM_TJPARAM_OPTIMIZE, optimize as libc::c_int)
    }

    /// Enable/disable progressive compression.
    ///
    /// By default, progressive compression is disabled. When enabled, the DCT coefficients will be
    /// encoded in multiple "scans": a low-quality version of the image represented with
    /// low-frequency DCT coefficients is encoded first, and subsequent scans refine it with
    /// higher-frequency DCT coefficients.
    ///
    /// Progressive encoding typically achieves higher compression ratios at the cost of
    /// considerably slower compression and decompression.
    ///
    /// # Example
    ///
    /// ```
    /// let image = turbojpeg::Image::mandelbrot(500, 500, turbojpeg::PixelFormat::RGB);
    /// let mut compressor = turbojpeg::Compressor::new()?;
    ///
    /// compressor.set_progressive(false)?;
    /// let single_scan = compressor.compress_to_vec(image.as_deref())?;
    ///
    /// compressor.set_progressive(true)?;
    /// let progressive = compressor.compress_to_vec(image.as_deref())?;
    ///
    /// assert!(progressive.len() < single_scan.len());
    /// let progressive_header = turbojpeg::read_header(&progressive)?;
    /// assert!(progressive_header.is_progressive);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[doc(alias = "TJPARAM_PROGRESSIVE")]
    pub fn set_progressive(&mut self, progressive: bool) -> Result<()> {
        self.handle.set(raw::TJPARAM_TJPARAM_PROGRESSIVE, progressive as libc::c_int)
    }

    /// Enable/disable arithmetic entropy coding.
    ///
    /// By default, arithmetic entropy coding is disabled and Huffman entropy coding is used
    /// instead.
    ///
    /// Arithmetic entropy coding improves the compression ratio but it makes the compression and
    /// decompression considerably slower.
    ///
    /// # Example
    ///
    /// ```
    /// let image = turbojpeg::Image::mandelbrot(500, 500, turbojpeg::PixelFormat::RGB);
    /// let mut compressor = turbojpeg::Compressor::new()?;
    ///
    /// compressor.set_arithmetic(false)?;
    /// let huffman = compressor.compress_to_vec(image.as_deref())?;
    ///
    /// compressor.set_arithmetic(true)?;
    /// let arithmetic = compressor.compress_to_vec(image.as_deref())?;
    ///
    /// assert!(arithmetic.len() < huffman.len());
    /// let arithmetic_header = turbojpeg::read_header(&arithmetic)?;
    /// assert!(arithmetic_header.is_arithmetic);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[doc(alias = "TJPARAM_ARITHMETIC")]
    pub fn set_arithmetic(&mut self, arithmetic: bool) -> Result<()> {
        self.handle.set(raw::TJPARAM_TJPARAM_ARITHMETIC, arithmetic as libc::c_int)
    }

    /// Compresses the `image` into `output` buffer.
    ///
    /// This is the main compression method, which gives you full control of the output buffer. If
    /// you don't need this level of control, you can use one of the convenience wrappers below.
    ///
    /// # Example
    ///
    /// ```
    /// // create an image (a Mandelbrot set visualization)
    /// let image = turbojpeg::Image::mandelbrot(500, 500, turbojpeg::PixelFormat::RGB);
    ///
    /// // initialize the compressor
    /// let mut compressor = turbojpeg::Compressor::new()?;
    /// compressor.set_quality(70);
    /// compressor.set_subsamp(turbojpeg::Subsamp::Sub2x2);
    ///
    /// // initialize the output buffer
    /// let mut output_buf = turbojpeg::OutputBuf::new_owned();
    ///
    /// // compress the image into JPEG
    /// // (we use as_deref() to convert from &Image<Vec<u8>> to Image<&[u8]>)
    /// compressor.compress(image.as_deref(), &mut output_buf)?;
    ///
    /// // write the JPEG to disk
    /// std::fs::write(std::env::temp_dir().join("mandelbrot.jpg"), &output_buf)?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[doc(alias = "tj3Compress8")]
    pub fn compress(&mut self, image: Image<&[u8]>, output: &mut OutputBuf) -> Result<()> {
        image.assert_valid(image.pixels.len());

        let Image { pixels, width, pitch, height, format } = image;
        let width = width.try_into().map_err(|_| Error::IntegerOverflow("width"))?;
        let pitch = pitch.try_into().map_err(|_| Error::IntegerOverflow("pitch"))?;
        let height = height.try_into().map_err(|_| Error::IntegerOverflow("height"))?;

        self.handle.set(
            raw::TJPARAM_TJPARAM_NOREALLOC,
            if output.is_owned { 0 } else { 1 } as libc::c_int,
        )?;
        let mut output_len = output.len as raw::size_t;
        let res = unsafe {
            raw::tj3Compress8(
                self.handle.as_ptr(),
                pixels.as_ptr(), width, pitch, height, format as libc::c_int,
                &mut output.ptr, &mut output_len,
            )
        };
        output.len = output_len as usize;
        if res != 0 {
            return Err(self.handle.get_error())
        } else if output.ptr.is_null() {
            output.len = 0;
            return Err(Error::Null)
        }

        Ok(())
    }

    /// Compresses the `image` into an owned buffer.
    ///
    /// This method automatically allocates the memory for output and avoids needless copying.
    pub fn compress_to_owned(&mut self, image: Image<&[u8]>) -> Result<OwnedBuf> {
        let mut buf = OutputBuf::new_owned();
        self.compress(image, &mut buf)?;
        Ok(buf.into_owned())
    }

    /// Compress the `image` into a new `Vec<u8>`.
    ///
    /// This method copies the compressed data into a new `Vec`. If you would like to avoid the
    /// extra allocation and copying, consider using [`compress_to_owned()`][Self::compress_to_owned]
    /// instead.
    pub fn compress_to_vec(&mut self, image: Image<&[u8]>) -> Result<Vec<u8>> {
        let mut buf = OutputBuf::new_owned();
        self.compress(image, &mut buf)?;
        Ok(buf.to_vec())
    }

    /// Compress the `image` into the slice `output`.
    ///
    /// Returns the size of the compressed JPEG data. If the compressed image does not fit into
    /// `dest`, this method returns an error. Use [`buf_len()`](Compressor::buf_len) to determine
    /// buffer size that is guaranteed to be large enough for the compressed image.
    pub fn compress_to_slice(&mut self, image: Image<&[u8]>, output: &mut [u8]) -> Result<usize> {
        let mut buf = OutputBuf::borrowed(output);
        self.compress(image, &mut buf)?;
        Ok(buf.len())
    }

    /// Compresses the [`YuvImage`] into `output` buffer.
    ///
    /// This is similar to [`compress()`][Self::compress], but encodes a YUV image instead of RGB
    /// image. This method gives you full control of the output buffer. If you don't need this
    /// level of control, you can use one of the convenience wrappers below.
    ///
    /// Encoding YUV images is useful if you already have an image in YUV, for example, if you
    /// receive it from a camera.
    ///
    /// # Example
    ///
    /// ```
    ///
    /// // parameters from rpi camera v3
    /// const WIDTH: usize = 1536;
    /// const HEIGHT: usize = 864;
    /// const SIZE: usize = WIDTH * HEIGHT * 3 / 2;
    ///
    /// // grab a raw yuv image from somewhere
    /// let mut raw_yuv_pixels: Vec<u8> = vec![0; SIZE];
    ///
    /// // initialize the compressor
    /// let mut compressor = turbojpeg::Compressor::new()?;
    /// compressor.set_quality(70);
    ///
    /// // initialize the YuvImage
    /// //  slice ref to avoid using as_deref
    /// let yuv_image = turbojpeg::YuvImage{
    ///     pixels: &raw_yuv_pixels[..],
    ///     width: WIDTH,
    ///     align: 4,
    ///     height: HEIGHT,
    ///     subsamp: turbojpeg::Subsamp::Sub2x2,
    ///  };
    ///
    /// // initialize the output buffer
    /// let mut output_buf = turbojpeg::OutputBuf::new_owned();
    ///
    /// // compress the image into JPEG
    /// compressor.compress_yuv(yuv_image, &mut output_buf).unwrap();
    ///
    /// // write the JPEG to disk
    /// std::fs::write(std::env::temp_dir().join("tj_encoded.jpg"), &output_buf)?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[doc(alias = "tj3CompressFromYUV8")]
    pub fn compress_yuv(&mut self, image: YuvImage<&[u8]>, output: &mut OutputBuf) -> Result<()> {
        image.assert_valid(image.pixels.len());

        let YuvImage { pixels, width, align, height, subsamp } = image;
        self.set_subsamp(subsamp)?;
        let width: libc::c_int = width.try_into().map_err(|_| Error::IntegerOverflow("width"))?;
        let align = align.try_into().map_err(|_| Error::IntegerOverflow("align"))?;
        let height: libc::c_int = height.try_into().map_err(|_| Error::IntegerOverflow("height"))?;

        self.handle.set(
            raw::TJPARAM_TJPARAM_NOREALLOC,
            if output.is_owned { 0 } else { 1 } as libc::c_int,
        )?;

        let mut output_len = output.len as raw::size_t;
        let res = unsafe {
            raw::tj3CompressFromYUV8(
                self.handle.as_ptr(),
                pixels.as_ptr(), width, align, height,
                &mut output.ptr, &mut output_len,
            )
        };
        output.len = output_len as usize;
        if res != 0 {
            return Err(self.handle.get_error())
        } else if output.ptr.is_null() {
            output.len = 0;
            return Err(Error::Null)
        }
        Ok(())
    }

    /// Compresses the [`YuvImage`] into an owned buffer.
    ///
    /// This method automatically allocates the memory for output and avoids needless copying.
    pub fn compress_yuv_to_owned(&mut self, image: YuvImage<&[u8]>) -> Result<OwnedBuf> {
        let mut buf = OutputBuf::new_owned();
        self.compress_yuv(image, &mut buf)?;
        Ok(buf.into_owned())
    }

    /// Compress the `YuvImage` into a new `Vec<u8>`.
    ///
    /// This method copies the compressed data into a new `Vec`. If you would like to avoid the
    /// extra allocation and copying, consider using
    /// [`compress_yuv_to_owned()`][Self::compress_yuv_to_owned] instead.
    pub fn compress_yuv_to_vec(&mut self, image: YuvImage<&[u8]>) -> Result<Vec<u8>> {
        let mut buf = OutputBuf::new_owned();
        self.compress_yuv(image, &mut buf)?;
        Ok(buf.to_vec())
    }

    /// Compress the `YuvImage` into the slice `output`.
    ///
    /// Returns the size of the compressed JPEG data. If the compressed image does not fit into
    /// `dest`, this method returns an error. Use [`compressed_buf_len()`] to determine buffer size
    /// that is guaranteed to be large enough for the compressed image.
    pub fn compress_yuv_to_slice(&mut self, image: YuvImage<&[u8]>, output: &mut [u8]) -> Result<usize> {
        let mut buf = OutputBuf::borrowed(output);
        self.compress_yuv(image, &mut buf)?;
        Ok(buf.len())
    }

    /// Compress separate YUV planes into JPEG.
    ///
    /// This function compresses separate Y, U (Cb), and V (Cr) image planes into
    /// a JPEG image. This is similar to [`compress_yuv()`][Self::compress_yuv],
    /// but takes separate YUV planes as input instead of interleaved YUV data.
    ///
    /// # Arguments
    ///
    /// * `image` - YUV image with separate planes
    /// * `output` - Output buffer for compressed JPEG data
    ///
    /// # Example
    ///
    /// ```
    /// // prepare YUV plane data
    /// let width = 640;
    /// let height = 480;
    /// let y_plane = vec![128u8; width * height];
    /// let u_plane = vec![128u8; (width/2) * (height/2)];
    /// let v_plane = vec![128u8; (width/2) * (height/2)];
    ///
    /// let image = turbojpeg::YuvPlanesImage {
    ///     y_plane: &y_plane[..],
    ///     u_plane: &u_plane[..],
    ///     v_plane: &v_plane[..],
    ///     width,
    ///     height,
    ///     y_stride: width,
    ///     u_stride: width/2,
    ///     v_stride: width/2,
    ///     subsamp: turbojpeg::Subsamp::Sub2x2,
    /// };
    ///
    /// // initialize the compressor
    /// let mut compressor = turbojpeg::Compressor::new()?;
    /// compressor.set_quality(90)?;
    ///
    /// // initialize the output buffer
    /// let mut output_buf = turbojpeg::OutputBuf::new_owned();
    ///
    /// // compress YUV planes to JPEG
    /// compressor.compress_yuv_planes(&image, &mut output_buf)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[doc(alias = "tj3CompressFromYUVPlanes8")]
    pub fn compress_yuv_planes(
        &mut self,
        image: &YuvPlanesImage<&[u8]>,
        output: &mut OutputBuf,
    ) -> Result<()> {
        image.assert_valid(image.y_plane.len(), image.u_plane.len(), image.v_plane.len());
        let width = image.width.try_into().map_err(|_| Error::IntegerOverflow("width"))?;
        let height = image.height.try_into().map_err(|_| Error::IntegerOverflow("height"))?;

        let planes = [
            image.y_plane.as_ptr(),
            image.u_plane.as_ptr(),
            image.v_plane.as_ptr(),
        ];

        let strides = [
            image.y_stride.try_into().map_err(|_| Error::IntegerOverflow("y_stride"))?,
            image.u_stride.try_into().map_err(|_| Error::IntegerOverflow("u_stride"))?,
            image.v_stride.try_into().map_err(|_| Error::IntegerOverflow("v_stride"))?,
        ];

        // Set subsampling from the YuvPlanesImage structure
        self.set_subsamp(image.subsamp)?;

        self.handle.set(
            raw::TJPARAM_TJPARAM_NOREALLOC,
            if output.is_owned { 0 } else { 1 } as libc::c_int,
        )?;

        let mut output_len = output.len as raw::size_t;
        let result = unsafe {
            raw::tj3CompressFromYUVPlanes8(
                self.handle.as_ptr(),
                planes.as_ptr(),
                width,
                strides.as_ptr(),
                height,
                &mut output.ptr,
                &mut output_len,
            )
        };
        output.len = output_len as usize;

        if result != 0 {
            return Err(self.handle.get_error());
        } else if output.ptr.is_null() {
            output.len = 0;
            return Err(Error::Null);
        }

        Ok(())
    }

    /// Compress separate YUV planes into an owned buffer.
    ///
    /// This method automatically allocates the memory for output and avoids needless copying.
    pub fn compress_yuv_planes_to_owned(
        &mut self,
        image: &YuvPlanesImage<&[u8]>,
    ) -> Result<OwnedBuf> {
        let mut buf = OutputBuf::new_owned();
        self.compress_yuv_planes(image, &mut buf)?;
        Ok(buf.into_owned())
    }

    /// Compress separate YUV planes into a new `Vec<u8>`.
    ///
    /// This method copies the compressed data into a new `Vec`. If you would like to avoid the
    /// extra allocation and copying, consider using
    /// [`compress_yuv_planes_to_owned()`][Self::compress_yuv_planes_to_owned] instead.
    pub fn compress_yuv_planes_to_vec(
        &mut self,
        image: &YuvPlanesImage<&[u8]>,
    ) -> Result<Vec<u8>> {
        let mut buf = OutputBuf::new_owned();
        self.compress_yuv_planes(image, &mut buf)?;
        Ok(buf.to_vec())
    }

    /// Compress separate YUV planes into the slice `output`.
    ///
    /// Returns the size of the compressed JPEG data. If the compressed image does not fit into
    /// `dest`, this method returns an error. Use [`compressed_buf_len()`] to determine buffer size
    /// that is guaranteed to be large enough for the compressed image.
    pub fn compress_yuv_planes_to_slice(
        &mut self,
        image: &YuvPlanesImage<&[u8]>,
        output: &mut [u8],
    ) -> Result<usize> {
        let mut buf = OutputBuf::borrowed(output);
        self.compress_yuv_planes(image, &mut buf)?;
        Ok(buf.len())
    }


    /// Compute the maximum size of a compressed image.
    ///
    /// This depends on image `width` and `height`, and also on the current setting of chrominance
    /// subsampling (see [`set_subsamp()`](Compressor::set_subsamp)).
    ///
    /// You can also use [`compressed_buf_len()`] directly.
    #[doc(alias = "tj3JPEGBufSize")]
    pub fn buf_len(&self, width: usize, height: usize) -> Result<usize> {
        compressed_buf_len(width, height, self.subsamp)
    }
}

/// Compress an image to JPEG.
///
/// Uses the given quality and chrominance subsampling option and returns the JPEG data in a buffer
/// owned by TurboJPEG. If this function does not fit your needs, please see [`Compressor`].
///
/// # Example
///
/// ```
/// // create an image (a Mandelbrot set visualization)
/// let image = turbojpeg::Image::mandelbrot(500, 500, turbojpeg::PixelFormat::RGB);
///
/// // compress the image into JPEG with quality 75 and in grayscale
/// // (we use as_deref() to convert from &Image<Vec<u8>> to Image<&[u8]>)
/// let jpeg_data = turbojpeg::compress(image.as_deref(), 75, turbojpeg::Subsamp::Sub2x2)?;
///
/// // write the JPEG to disk
/// std::fs::write(std::env::temp_dir().join("mandelbrot.jpg"), &jpeg_data)?;
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn compress(image: Image<&[u8]>, quality: i32, subsamp: Subsamp) -> Result<OwnedBuf> {
    let mut compressor = Compressor::new()?;
    compressor.set_quality(quality)?;
    compressor.set_subsamp(subsamp)?;
    compressor.compress_to_owned(image)
}

/// Compress a YUV image to JPEG.
///
/// Uses the given quality and returns the JPEG data in a buffer owned by TurboJPEG. If this
/// function does not fit your needs, please see [`Compressor`].
///
/// # Example
///
/// ```
/// // obtain an YUV image
/// let orig_data = std::fs::read("examples/parrots.jpg")?;
/// let yuv_image = turbojpeg::decompress_to_yuv(&orig_data)?;
///
/// // compress the image into JPEG with quality 90
/// // (we use as_deref() to convert from &Image<Vec<u8>> to Image<&[u8]>)
/// let jpeg_data = turbojpeg::compress_yuv(yuv_image.as_deref(), 90)?;
///
/// // write the JPEG to disk
/// std::fs::write(std::env::temp_dir().join("same_parrots.jpg"), &jpeg_data)?;
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn compress_yuv(image: YuvImage<&[u8]>, quality: i32) -> Result<OwnedBuf> {
    let mut compressor = Compressor::new()?;
    compressor.set_quality(quality)?;
    compressor.compress_yuv_to_owned(image)
}

/// Compress separate YUV planes to JPEG.
///
/// Uses the given quality and returns the JPEG data in a buffer owned by TurboJPEG. If this
/// function does not fit your needs, please see [`Compressor`].
///
/// # Example
///
/// ```
/// // prepare YUV plane data
/// let width = 640;
/// let height = 480;
/// let y_plane = vec![128u8; width * height];
/// let u_plane = vec![128u8; (width/2) * (height/2)];
/// let v_plane = vec![128u8; (width/2) * (height/2)];
///
/// let image = turbojpeg::YuvPlanesImage {
///     y_plane: &y_plane[..],
///     u_plane: &u_plane[..],
///     v_plane: &v_plane[..],
///     width,
///     height,
///     y_stride: width,
///     u_stride: width/2,
///     v_stride: width/2,
///     subsamp: turbojpeg::Subsamp::Sub2x2,
/// };
///
/// // compress the YUV planes into JPEG with quality 90
/// let jpeg_data = turbojpeg::compress_yuv_planes(&image, 90)?;
///
/// // write the JPEG to disk
/// std::fs::write(std::env::temp_dir().join("yuv_planes.jpg"), &jpeg_data)?;
///
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn compress_yuv_planes(image: &YuvPlanesImage<&[u8]>, quality: i32) -> Result<OwnedBuf> {
    let mut compressor = Compressor::new()?;
    compressor.set_quality(quality)?;
    compressor.compress_yuv_planes_to_owned(image)
}

/// Compute the maximum size of a compressed image.
///
/// This depends on image `width` and `height` and also on the chrominance subsampling method.
///
/// Returns an error on integer overflow. You can just `.unwrap()` the result if you don't care
/// about this edge case.
#[doc(alias = "tj3JPEGBufSize")]
pub fn compressed_buf_len(width: usize, height: usize, subsamp: Subsamp) -> Result<usize> {
    let width = width.try_into().map_err(|_| Error::IntegerOverflow("width"))?;
    let height = height.try_into().map_err(|_| Error::IntegerOverflow("height"))?;
    let len = unsafe { raw::tj3JPEGBufSize(width, height, subsamp as libc::c_int) };
    let len = len.try_into().map_err(|_| Error::IntegerOverflow("buf len"))?;
    Ok(len)
}
