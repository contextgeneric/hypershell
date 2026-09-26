use core::fmt::{self, Debug, Display};
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, ready};
use std::io::{self, ErrorKind};
use std::process::ExitStatus;
use std::sync::{Arc, Mutex};

use tokio::io::{AsyncRead, AsyncReadExt, ReadBuf, copy};
use tokio::process::{Child, ChildStdout};
use tokio::spawn;
use tokio::task::JoinHandle;

/// The standard output of a streamed child process, which reports the child's failure when it ends.
///
/// Reading passes the child's stdout through, while background tasks feed the input to the child's
/// stdin and drain its stderr, so a child writing a lot to stderr cannot block. At end of file the
/// reader waits for the child to exit. An error reading the input, such as a failure the previous
/// stage reports, ends this stream with that error; otherwise a non-success exit is returned as an
/// [`io::Error`] carrying a [`ChildExitError`]. Either way the stage that reads the stream fails
/// instead of treating truncated output as complete.
pub struct ChildOutputStream {
    stdout: Option<ChildStdout>,
    feed: Option<JoinHandle<()>>,
    input_error: Arc<Mutex<Option<io::Error>>>,
    exit: Option<JoinHandle<io::Result<ChildExit>>>,
}

struct ChildExit {
    status: ExitStatus,
    stderr: Vec<u8>,
}

/// A streamed child process that exited with a non-success status, with what it wrote to standard
/// error.
pub struct ChildExitError {
    pub status: ExitStatus,
    pub stderr: Vec<u8>,
}

impl ChildOutputStream {
    /// Take the child's pipes, and start the task that copies `input` to its stdin and the task that
    /// drains its stderr and waits for it to exit.
    pub fn new<Input>(mut child: Child, mut input: Input) -> Self
    where
        Input: AsyncRead + Send + Unpin + 'static,
    {
        let stdout = child.stdout.take();
        let mut stderr = child.stderr.take();
        let input_error = Arc::new(Mutex::new(None));

        let feed = child.stdin.take().map(|mut stdin| {
            let input_error = input_error.clone();
            spawn(async move {
                match copy(&mut input, &mut stdin).await {
                    // A child that exits before reading all of its input closes the pipe, which is
                    // not a failure of the input.
                    Err(e) if e.kind() != ErrorKind::BrokenPipe => {
                        if let Ok(mut slot) = input_error.lock() {
                            *slot = Some(e);
                        }
                    }
                    _ => {}
                }
                // Closing stdin only after recording the error means that a child which exits
                // because its input ended can never be observed before the error is.
                drop(stdin);
            })
        });

        let exit = spawn(async move {
            let mut stderr_bytes = Vec::new();
            if let Some(stderr) = &mut stderr {
                stderr.read_to_end(&mut stderr_bytes).await?;
            }

            let status = child.wait().await?;

            Ok(ChildExit {
                status,
                stderr: stderr_bytes,
            })
        });

        Self {
            stdout,
            feed,
            input_error,
            exit: Some(exit),
        }
    }
}

impl AsyncRead for ChildOutputStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        if buf.remaining() == 0 {
            return Poll::Ready(Ok(()));
        }

        if let Some(stdout) = &mut self.stdout {
            let filled = buf.filled().len();
            ready!(Pin::new(stdout).poll_read(cx, buf))?;

            if buf.filled().len() > filled {
                return Poll::Ready(Ok(()));
            }

            // End of file on stdout: the input and the child's exit decide how the stream ends.
            self.stdout = None;
        }

        if let Some(exit) = &mut self.exit {
            let result = ready!(Pin::new(exit).poll(cx));
            self.exit = None;

            let exit = result.map_err(io::Error::other)?;

            // The child has exited, so a feed still running is blocked on input nobody will read.
            if let Some(feed) = self.feed.take() {
                feed.abort();
            }

            // A failed input is the root cause of whatever the child did with it.
            let input_error = self
                .input_error
                .lock()
                .ok()
                .and_then(|mut slot| slot.take());
            if let Some(error) = input_error {
                return Poll::Ready(Err(error));
            }

            let ChildExit { status, stderr } = exit?;

            if !status.success() {
                return Poll::Ready(Err(io::Error::other(ChildExitError { status, stderr })));
            }
        }

        Poll::Ready(Ok(()))
    }
}

impl Display for ChildExitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "child process exited with non-success code {:?}, stderr: {}",
            self.status.code(),
            String::from_utf8_lossy(&self.stderr),
        )
    }
}

impl Debug for ChildExitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl std::error::Error for ChildExitError {}
