use crate::log::{Validate, Logger};
use crate::error::Error;
use crate::error::severity::Severity;

pub trait Id {
    type Identifier: Eq;

    fn id(&self) -> Self::Identifier;
}

pub trait ValidateUnique {
    type Aux;

    fn validate_unique(&self, _: &mut Logger, _: &Self::Aux) -> Result<(), Error>;
}

impl<T> ValidateUnique for Vec<T>
    where
        T: Validate + Id
{
    type Aux = T::Aux;

    fn validate_unique(&self, logger: &mut Logger, aux: &Self::Aux) -> Result<(), Error> {
        let mut uniques = Vec::new();
        for val in self.iter() {
            if uniques.contains(&val.id()) {
                let desc = format!("Unique item declared twice.");
                logger.log(Severity::Critical, &desc);
                Err(Error::DuplicateModule("temp".to_owned()))?;
            } else {
                val.validate(logger, aux)?;

                uniques.push(val.id());
            }
        }

        Ok(())
    }
}