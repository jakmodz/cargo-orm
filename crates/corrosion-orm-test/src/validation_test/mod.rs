use corrosion_orm_macros::Validate;

#[derive(Validate)]
#[allow(dead_code)]
struct User {
    #[NotNull]
    pub name: String,
}
#[derive(Validate)]
#[allow(dead_code)]
struct Email {
    #[Email]
    pub email: String,
}
#[derive(Validate)]
#[allow(dead_code)]
struct Phone {
    #[NotNull]
    #[Pattern(pattern = "^\\d{10}$")]
    pub phone: String,
}
#[derive(Validate)]
#[allow(dead_code)]
pub struct ComplexValidation {
    #[Size(max = 10)]
    pub name: String,
    #[NotNull]
    #[Pattern(pattern = "^\\d{10}$")]
    pub phone: String,
    #[Email]
    pub email: String,
    #[NotNull]
    pub age: String,
}
#[cfg(test)]
mod tests {
    use corrosion_orm_core::validation::ValidationError;

    use crate::validation_test::{ComplexValidation, Email, Phone, User};

    #[test]
    fn test_username_is_null() {
        let user = User {
            name: "".to_string(),
        };
        assert!(
            user.validate().is_err(),
            "Empty name should fail NotNull validation"
        );
    }

    #[test]
    fn test_phone_not_null() {
        let user = User {
            name: "".to_string(),
        };
        assert!(
            user.validate().is_err(),
            "Empty name should fail NotNull validation"
        );
    }

    #[test]
    fn test_invalid_phone() -> Result<(), ValidationError> {
        let phone = Phone {
            phone: "123".to_string(),
        };
        assert!(phone.validate().is_err());
        Ok(())
    }
    #[test]
    fn test_valid_phone() -> Result<(), ValidationError> {
        let phone = Phone {
            phone: "1234567890".to_string(),
        };
        phone.validate()?;
        Ok(())
    }

    #[test]
    fn test_invalid_email() {
        let email = Email {
            email: "invalid_email".to_string(),
        };
        assert!(
            email.validate().is_err(),
            "invalid_email should fail email validation"
        );
    }
    #[test]
    fn test_valid_email() -> Result<(), ValidationError> {
        let email = Email {
            email: "valid_email@example.com".to_string(),
        };
        email.validate()?;
        Ok(())
    }
    #[test]
    fn test_invalid_complex() -> Result<(), ValidationError> {
        let user = ComplexValidation {
            name: "".to_string(),
            phone: "123".to_string(),
            email: "invalid_email".to_string(),
            age: "".to_string(),
        };
        dbg!(user.validate().is_err());
        assert!(user.validate().is_err());
        Ok(())
    }
    #[test]
    fn test_valid_complex() -> Result<(), ValidationError> {
        let user = ComplexValidation {
            name: "John".to_string(),
            phone: "1234567890".to_string(),
            email: "valid_email@example.com".to_string(),
            age: "30".to_string(),
        };
        user.validate()?;
        Ok(())
    }
}
