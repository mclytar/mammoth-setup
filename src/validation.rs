use std::path::Path;

use crate::error::Error;
use crate::error::severity::Severity;
use crate::log::Logger;
use crate::id::Id;
use serde::export::PhantomData;

pub trait Validator<T> {
    fn validate(&self, _: &mut Logger, _: &T) -> Result<(), Error>;
}

#[derive(Copy, Clone)]
pub enum PathValidatorKind {
    DirectoryPath,
    ExistingFile,
    FilePath,
}

#[derive(Copy, Clone)]
pub struct PathValidator(pub Severity, pub PathValidatorKind);

impl Validator<&Path> for PathValidator {
    fn validate(&self, logger: &mut Logger, item: &&Path) -> Result<(), Error> {
        let severity = self.0;
        let data = self.1;

        match data {
            PathValidatorKind::DirectoryPath => if !item.is_dir() || !item.exists() {
                let desc = format!("Not a valid directory: '{:?}'.", item);
                logger.log(severity, &desc);
                if severity >= Severity::Error { Err(Error::InvalidDirectory(item.to_path_buf()))?; }
            },
            PathValidatorKind::ExistingFile => if !item.is_file() || !item.exists() {
                let desc = format!("File does not exists: '{:?}'.", item);
                logger.log(severity, &desc);
                if severity >= Severity::Error { Err(Error::FileNotFound(item.to_path_buf()))?; }
            }
            PathValidatorKind::FilePath => if !item.is_file() {
                let desc = format!("Not a valid file path: '{:?}'.", item);
                logger.log(severity, &desc);
                if severity >= Severity::Error { Err(Error::InvalidFilePath(item.to_path_buf()))?; }
            }
        }

        Ok(())
    }
}

impl<T> Validator<T> for Fn(&mut Logger, &T) -> Result<(), Error> {
    fn validate(&self, logger: &mut Logger, item: &T) -> Result<(), Error> {
        self(logger, item)
    }
}

pub struct IdValidator<I: Id, V: Validator<I>> (pub Severity, pub V, pub PhantomData<I>);

impl<I, V> Validator<Vec<I>> for IdValidator<I, V>
    where
        I: Id,
        V: Validator<I>
{
    fn validate(&self, logger: &mut Logger, item: &Vec<I>) -> Result<(), Error> {
        let mut uniques = Vec::new();

        for val in item {
            if uniques.contains(&val.id()) || uniques.contains(&val.id()) {
                let desc = format!("Unique item declared twice.");
                logger.log(self.0, &desc);
                Err(Error::DuplicateItem("temp".to_owned()))?;
            } else {
                self.1.validate(logger, val)?;

                uniques.push(val.id());
            }
        }

        Ok(())
    }
}

impl<I, V> Validator<Vec<&I>> for IdValidator<I, V>
    where
        I: Id,
        V: Validator<I>
{
    fn validate(&self, logger: &mut Logger, item: &Vec<&I>) -> Result<(), Error> {
        let mut uniques = Vec::new();

        for &val in item {
            if uniques.contains(&val.id()) || uniques.contains(&val.id()) {
                let desc = format!("Unique item declared twice.");
                logger.log(self.0, &desc);
                Err(Error::DuplicateItem("temp".to_owned()))?;
            } else {
                self.1.validate(logger, val)?;

                uniques.push(val.id());
            }
        }

        Ok(())
    }
}