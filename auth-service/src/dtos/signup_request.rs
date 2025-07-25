use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess, Unexpected};
use std::fmt;
use regex::Regex;
use once_cell::sync::Lazy;

static EMAIL_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?xi)                # enable case-insensitive & ignore-whitespace
        ^[A-Z0-9._%+-]+              # local-part
        @
        [A-Z0-9-]+                   # domain name
        (?:\.[A-Z0-9-]+)*            # optional subdomains
        \.[A-Z]{2,}                  # top-level domain
        $
    ").unwrap()
});

pub fn is_valid_email(email: &str) -> bool {
    EMAIL_RE.is_match(email)
}

pub struct SignupRequestBody {
    pub email: String,
    pub password: String,
    pub requires_mfa: bool
}

impl Serialize for SignupRequestBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut state = serializer.serialize_struct("SignupBody", 3)?;
        state.serialize_field("email", &self.email)?;
        state.serialize_field("password", &self.password)?;
        state.serialize_field("requires2fa", &self.requires_mfa)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for SignupRequestBody {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Email, Password, Requires2FA };
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`email`, `password` or `requires2FA`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "email" => Ok(Field::Email),
                            "password" => Ok(Field::Password),
                            "requires2fa" => Ok(Field::Requires2FA),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct DurationVisitor;

        impl<'de> Visitor<'de> for DurationVisitor {
            type Value = SignupRequestBody;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct SignupRequestBody")
            }

            fn visit_map<V>(self, mut map: V) -> Result<SignupRequestBody, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut email: Option<String> = None;
                let mut password: Option<String> = None;
                let mut requires_mfa: Option<bool> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Email => {
                            if email.is_some() {
                                return Err(de::Error::duplicate_field("email"));
                            }
                            email = Some(map.next_value()?);
                            if !is_valid_email(&email.as_ref().unwrap()) {
                                return Err(de::Error::invalid_value(Unexpected::Str(&email.as_ref().unwrap()), &"`valid email address`"));
                            }
                        }
                        Field::Password => {
                            if password.is_some() {
                                return Err(de::Error::duplicate_field("password"));
                            }
                            password = Some(map.next_value()?);
                            let min_password_len: usize = 8;
                            let password_len = &password.as_ref().unwrap().len();
                            if  password_len < &min_password_len {
                                return Err(de::Error::invalid_length(*password_len, &"`password should be at least 8 chars long`"));
                            }
                        }
                        Field::Requires2FA => {
                            if requires_mfa.is_some() {
                                return Err(de::Error::duplicate_field("requires2fa"));
                            }
                            requires_mfa = Some(map.next_value()?);
                        }
                    }
                }
                let email = email.ok_or_else(|| de::Error::missing_field("email"))?;
                let password = password.ok_or_else(|| de::Error::missing_field("password"))?;
                let requires_mfa = requires_mfa.ok_or_else(|| de::Error::missing_field("requires2fa"))?;
                Ok(SignupRequestBody {
                    email,
                    password,
                    requires_mfa
                })
            }
        }

        const FIELDS: &[&str] = &["email", "password", "requires2fa"];
        deserializer.deserialize_struct("SignupRequestBody", FIELDS, DurationVisitor)
    }
}