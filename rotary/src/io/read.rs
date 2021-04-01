use rotary_core::{Buf, Channel, Channels, ExactSizeBuf, ReadBuf};

/// Make a buffer into a read adapter that implements [ReadBuf].
///
/// # Examples
///
/// ```rust
/// use rotary::Buf as _;
/// use rotary::io;
///
/// let from = rotary::interleaved![[1, 2, 3, 4]; 2];
/// let mut to = rotary::interleaved![[0; 4]; 2];
///
/// let mut to = io::ReadWrite::new(to);
///
/// io::copy_remaining(io::Read::new((&from).skip(2).limit(1)), &mut to);
/// io::copy_remaining(io::Read::new((&from).limit(1)), &mut to);
///
/// assert_eq!(to.as_ref().as_slice(), &[3, 3, 1, 1, 0, 0, 0, 0]);
/// ```
pub struct Read<B> {
    buf: B,
    available: usize,
}

impl<B> Read<B>
where
    B: ExactSizeBuf,
{
    /// Construct a new read adapter.
    pub fn new(buf: B) -> Self {
        let available = buf.frames();
        Self { buf, available }
    }

    /// Access the underlying buffer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rotary::Buf as _;
    /// use rotary::{io, wrap};
    ///
    /// let from: rotary::Interleaved<i16> = rotary::interleaved![[1, 2, 3, 4]; 4];
    /// let mut from = io::Read::new(from);
    ///
    /// io::copy_remaining(&mut from, wrap::interleaved(&mut [0i16; 16][..], 4));
    ///
    /// assert_eq!(from.as_ref().channels(), 4);
    /// ```
    pub fn as_ref(&self) -> &B {
        &self.buf
    }

    /// Access the underlying buffer mutably.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rotary::Buf as _;
    /// use rotary::{io, wrap};
    ///
    /// let from: rotary::Interleaved<i16> = rotary::interleaved![[1, 2, 3, 4]; 4];
    /// let mut from = io::Read::new(from);
    ///
    /// io::copy_remaining(&mut from, wrap::interleaved(&mut [0i16; 16][..], 4));
    ///
    /// from.as_mut().resize_channels(2);
    ///
    /// assert_eq!(from.channels(), 2);
    /// ```
    pub fn as_mut(&mut self) -> &mut B {
        &mut self.buf
    }

    /// Convert into the underlying buffer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rotary::Buf as _;
    /// use rotary::{io, wrap};
    ///
    /// let from: rotary::Interleaved<i16> = rotary::interleaved![[1, 2, 3, 4]; 4];
    /// let mut from = io::Read::new(from);
    ///
    /// io::copy_remaining(&mut from, wrap::interleaved(&mut [0i16; 16][..], 4));
    ///
    /// let from = from.into_inner();
    ///
    /// assert_eq!(from.channels(), 4);
    /// ```
    pub fn into_inner(self) -> B {
        self.buf
    }

    /// Set the number of frames read.
    pub fn set_read(&mut self, read: usize) {
        self.available = self.buf.frames().saturating_sub(read);
    }
}

impl<B> ReadBuf for Read<B> {
    fn remaining(&self) -> usize {
        self.available
    }

    fn advance(&mut self, n: usize) {
        self.available = self.available.saturating_sub(n);
    }
}

impl<B> ExactSizeBuf for Read<B>
where
    B: ExactSizeBuf,
{
    fn frames(&self) -> usize {
        self.buf.frames().saturating_sub(self.available)
    }
}

impl<B> Buf for Read<B>
where
    B: Buf,
{
    fn frames_hint(&self) -> Option<usize> {
        self.buf.frames_hint()
    }

    fn channels(&self) -> usize {
        self.buf.channels()
    }
}

impl<B, T> Channels<T> for Read<B>
where
    B: Channels<T>,
{
    fn channel(&self, channel: usize) -> Channel<'_, T> {
        self.buf.channel(channel).tail(self.available)
    }
}
