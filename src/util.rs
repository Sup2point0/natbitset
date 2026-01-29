/// Construct and return a boxed error with the given format string message.
macro_rules! boxerr {
    ($error:expr => $message:expr) =>
    {
        return Err(
            std::boxed::Box::new(
                $error(
                    format!($message)
                )
            )
        )
    };
}

pub(crate) use boxerr;
