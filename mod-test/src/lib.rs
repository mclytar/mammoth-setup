use mammoth_setup::prelude::*;
use mammoth_setup::error::severity::Severity;

#[mammoth_module(constructor_fn)]
pub struct TestModule {
    test: Option<Value>,
    logger: Option<AsyncLoggerReference>
}

fn constructor_fn(test: Option<Value>) -> TestModule {
    TestModule {
        test,
        logger: None
    }
}

impl MammothInterface for TestModule {
    fn on_load(&self) {
        self.log(Severity::Debug, "Test module loaded.");
    }

    fn on_validation(&self, logger: &mut Logger) -> Result<(), Error> {
        if let Some(ref value) = self.test {
            if value.is_str() {
                if value.as_str().unwrap() == "test_error" {
                    logger.log(Severity::Debug, "Error tested successfully!");
                    Err(Error::Unknown)
                } else {
                    Ok(())
                }
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    fn on_shutdown(&self) {
        self.log(Severity::Debug, "Test module unloaded.");
    }
}

impl Log for TestModule {
    fn register_logger(&mut self, logger: AsyncLoggerReference) {
        self.logger = Some(logger.clone());
    }

    fn retrieve_logger(&self) -> Option<AsyncLoggerReference> {
        self.logger.clone()
    }

    fn log(&self, sev: Severity, desc: &str) {
        if let Some(ref logger) = self.logger {
            let mut logger = logger.write().unwrap();

            logger.log(sev, desc);
        }
    }
}