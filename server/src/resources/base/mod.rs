//! Base

pub mod linked_resource;
pub mod simple_resource;
use crate::grpc::server::{Id, IdList};
use crate::postgres::PsqlJsonValue;
use crate::{common::ArrErr, grpc::GrpcDataObjectType};
use core::fmt::Debug;
use log::error;
use std::collections::HashMap;
use tokio_postgres::types::Type as PsqlFieldType;
use uuid::Uuid;

#[cfg(test)]
pub mod test_util;

/// Generic trait providing useful functions for our resources
pub trait Resource
where
    Self: Sized,
{
    /// Allows us to implement the resource definition used for simple insert and update queries
    fn get_definition() -> ResourceDefinition;

    /// This function should be implemented for the resources where applicable (example implementation can be found in the flight_plan module).
    fn get_enum_string_val(field: &str, value: i32) -> Option<String> {
        let _field = field;
        let _value = value;
        None
    }
    /// This function should be implemented for the resources where applicable (example implementation can be found in the flight_plan module).
    fn get_table_indices() -> Vec<String> {
        vec![]
    }
    /// Returns `true` if the given column name is part of the resource's combined id
    fn has_id_col(id_col: &str) -> bool {
        for col in Self::get_definition().get_psql_id_cols() {
            if col == id_col {
                return true;
            }
        }
        false
    }
    /// Returns the `psql_table` [String] value of the resource's [ResourceDefinition]
    fn get_psql_table() -> String {
        Self::get_definition().get_psql_table()
    }
}

/// Allows us to transform the gRPC `Object` structs into a generic object
pub trait ObjectType<T>
where
    Self: Resource,
    T: GrpcDataObjectType,
{
    /// Get [`ObjectType<T>`]'s `ids` field, to be implemented by trait implementor
    fn get_ids(&self) -> Option<HashMap<String, String>> {
        None
    }
    /// Get [`ObjectType<T>`]'s `data` field, to be implemented by trait implementor
    fn get_data(&self) -> Option<T> {
        None
    }
    /// Set [`ObjectType<T>`]'s `ids` field, to be implemented by trait implementor
    fn set_ids(&mut self, ids: HashMap<String, String>);
    /// Set [`ObjectType<T>`]'s `data` field, to be implemented by trait implementor
    fn set_data(&mut self, data: T);

    /// Returns [`ObjectType<T>`]'s `data` [`GrpcDataObjectType`] value
    ///
    /// # Errors
    ///
    /// Returns [`ArrErr`] if any of the provided `id` [`String`]s could not be converted to a valid [`Uuid`]
    /// Get `Object` `data` if set, returns [`ArrErr`] if no `data` is set
    fn try_get_data(&self) -> Result<T, ArrErr> {
        match self.get_data() {
            Some(data) => Ok(data),
            None => {
                let error =
                    "No data provided for ObjectType<T> when calling [try_get_data]".to_string();
                error!("{}", error);
                Err(ArrErr::Error(error))
            }
        }
    }
    /// Returns [`ObjectType<T>`]'s `ids` [`HashMap<String, String>`] as [`HashMap<String, Uuid>`]
    ///
    /// # Errors
    ///
    /// Returns [`ArrErr`] if any of the provided `id` [`String`]s could not be converted to a valid [`Uuid`]
    fn try_get_uuids(&self) -> Result<HashMap<String, Uuid>, ArrErr> {
        match self.get_ids() {
            Some(ids) => {
                let mut result = HashMap::new();
                for (field, id) in ids {
                    let uuid = Uuid::parse_str(&id)?;
                    result.insert(field, uuid);
                }
                Ok(result)
            }
            None => Err(ArrErr::Error(format!(
                "No ids configured for resource [{}]",
                Self::get_psql_table()
            ))),
        }
    }
    /// Returns [`ObjectType<T>`]'s `id_field` value as [`Option<String>`] if found
    ///
    /// Returns [`None`] if `ids` is not set, or the `id_field` is not found as a key in the `ids` [`HashMap`]
    fn get_value_for_id_field(&self, id_field: &str) -> Option<String> {
        match self.get_ids() {
            Some(map) => map.get(id_field).cloned(),
            None => None,
        }
    }
}

/// struct object defining resource metadata
#[derive(Clone, Debug)]
pub struct ResourceDefinition {
    /// psql table corresponding to the resource
    pub psql_table: String,
    /// psql column names used to identify the unique resource in the database
    pub psql_id_cols: Vec<String>,
    /// resource fields definition
    pub fields: HashMap<String, FieldDefinition>,
}

impl ResourceDefinition {
    /// returns [`String`] value of the struct's `psql_table` field
    pub fn get_psql_table(&self) -> String {
        self.psql_table.clone()
    }

    /// returns [`Vec<String>`] value of the struct's `psql_table_ids` field
    pub fn get_psql_id_cols(&self) -> Vec<String> {
        self.psql_id_cols.clone()
    }

    /// returns [`bool`] true if the provided `field` key is found in the `fields` [`HashMap`]
    pub fn has_field(&self, field: &str) -> bool {
        self.fields.contains_key(field)
    }

    /// returns [`FieldDefinition`] if the provided `field` is found in the `fields` [`HashMap`]
    /// returns an [`ArrErr`] if the field does not exist
    pub fn try_get_field(&self, field: &str) -> Result<&FieldDefinition, ArrErr> {
        match self.fields.get(field) {
            Some(field) => Ok(field),
            None => Err(ArrErr::Error(format!(
                "Tried to get field [{}] for table [{}], but the field does not exist.",
                field, self.psql_table
            ))),
        }
    }
}

/// Generic resource wrapper struct used to implement our generic traits
#[derive(Clone, Debug)]
pub struct ResourceObject<T>
where
    T: GrpcDataObjectType + prost::Message,
{
    /// unique ids of the resource [`HashMap<String, String>`]
    pub ids: Option<HashMap<String, String>>,
    /// resource field data
    pub data: Option<T>,
    /// field mask used for update actions
    pub mask: Option<::prost_types::FieldMask>,
}
impl<T: GrpcDataObjectType + prost::Message> ObjectType<T> for ResourceObject<T>
where
    Self: Resource,
{
    fn get_ids(&self) -> Option<HashMap<String, String>> {
        self.ids.clone()
    }
    fn set_ids(&mut self, ids: HashMap<String, String>) {
        self.ids = Some(ids)
    }
    fn get_data(&self) -> Option<T> {
        self.data.clone()
    }
    fn set_data(&mut self, data: T) {
        self.data = Some(data)
    }
}

/// Field definition struct defining field properties
#[derive(Clone, Debug)]
pub struct FieldDefinition {
    /// [`PsqlFieldType`]
    pub field_type: PsqlFieldType,
    /// [`bool`] to set if field is mandatory in the database
    mandatory: bool,
    /// [`bool`] to set if field should not be exposed to gRPC object
    internal: bool,
    /// [`bool`] to set if field should be read only for clients
    read_only: bool,
    /// [`String`] option to provide a default value used during database inserts
    default: Option<String>,
}

impl FieldDefinition {
    /// Create a new [`FieldDefinition`] with provided field_type and mandatory setting
    pub fn new(field_type: PsqlFieldType, mandatory: bool) -> Self {
        Self {
            field_type,
            mandatory,
            internal: false,
            read_only: false,
            default: None,
        }
    }
    /// Create a new internal [`FieldDefinition`] with provided field_type and mandatory setting
    pub fn new_internal(field_type: PsqlFieldType, mandatory: bool) -> Self {
        Self {
            field_type,
            mandatory,
            internal: true,
            read_only: true,
            default: None,
        }
    }
    /// Create a new read_only [`FieldDefinition`] with provided field_type and mandatory setting
    pub fn new_read_only(field_type: PsqlFieldType, mandatory: bool) -> Self {
        Self {
            field_type,
            mandatory,
            internal: false,
            read_only: true,
            default: None,
        }
    }

    /// Returns [`bool`] mandatory
    pub fn is_mandatory(&self) -> bool {
        self.mandatory
    }
    /// Returns [`bool`] internal
    pub fn is_internal(&self) -> bool {
        self.internal
    }
    /// Returns [`bool`] internal
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    /// Returns [`bool`] `true` if a `default` value has been provided for this field and `false`if not
    pub fn has_default(&self) -> bool {
        self.default.is_some()
    }
    /// Sets the `default` value using the given default [`String`]
    pub fn set_default(&mut self, default: String) -> Self {
        self.default = Some(default);
        self.clone()
    }
    /// Gets the `default` value for this field
    ///
    /// The function will panic if no default has been set. It's recommended to call
    /// [`has_default`](FieldDefinition::has_default) first, to determine if this function can be used or
    /// not
    pub fn get_default(&self) -> String {
        if self.has_default() {
            self.default.clone().unwrap_or_else(|| String::from("NULL"))
        } else {
            panic!("get_default called on a field without a default value");
        }
    }
}

impl TryFrom<Id> for Uuid {
    type Error = ArrErr;
    fn try_from(id: Id) -> Result<Self, ArrErr> {
        Uuid::try_parse(&id.id).map_err(ArrErr::UuidError)
    }
}
impl TryFrom<IdList> for Vec<Uuid> {
    type Error = ArrErr;
    fn try_from(list: IdList) -> Result<Self, ArrErr> {
        let mut uuid_list = vec![];
        for id in list.ids.iter() {
            uuid_list.push(Uuid::try_parse(id).map_err(ArrErr::UuidError)?);
        }
        Ok(uuid_list)
    }
}

impl TryFrom<PsqlJsonValue> for Vec<u32> {
    type Error = ArrErr;
    fn try_from(json_value: PsqlJsonValue) -> Result<Self, ArrErr> {
        match json_value.value.as_array() {
            Some(arr) => {
                let iter = arr.iter();
                let mut vec: Vec<u32> = vec![];
                for val in iter {
                    vec.push(val.as_u64().ok_or(ArrErr::Error(format!(
                        "json_value did not contain array with u32: {}",
                        json_value.value
                    )))? as u32);
                }
                Ok(vec)
            }
            None => {
                let error = format!(
                    "Could not convert [PsqlJsonValue] to [Vec<u32>]: {:?}",
                    json_value
                );
                error!("{}", error);
                Err(ArrErr::Error(error))
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use tokio_postgres::types::Type as PsqlFieldType;

    // FieldDefinition tests
    #[test]
    fn test_field_definition_new() {
        let field_type = PsqlFieldType::VARCHAR;
        let mandatory = true;
        let field_def = FieldDefinition::new(field_type.clone(), mandatory);

        assert_eq!(field_def.field_type, field_type);
        assert_eq!(field_def.is_mandatory(), mandatory);
        assert!(!field_def.is_internal());
        assert!(!field_def.is_read_only());
        assert!(!field_def.has_default());
    }

    #[test]
    fn test_field_definition_internal_field() {
        let field_type = PsqlFieldType::FLOAT8;
        let mandatory = false;
        let field_def = FieldDefinition::new_internal(field_type.clone(), mandatory);

        assert_eq!(field_def.field_type, field_type);
        assert_eq!(field_def.is_mandatory(), mandatory);
        assert!(field_def.is_internal());
        assert!(field_def.is_read_only());
        assert!(!field_def.has_default());
    }

    #[test]
    fn test_field_definition_read_only_field() {
        let field_type = PsqlFieldType::FLOAT8;
        let mandatory = false;
        let field_def = FieldDefinition::new_read_only(field_type.clone(), mandatory);

        assert_eq!(field_def.field_type, field_type);
        assert_eq!(field_def.is_mandatory(), mandatory);
        assert!(!field_def.is_internal());
        assert!(field_def.is_read_only());
        assert!(!field_def.has_default());
    }

    #[test]
    fn test_field_definition_set_default() {
        let field_type = PsqlFieldType::BOOL;
        let mandatory = true;
        let mut field_def = FieldDefinition::new(field_type, mandatory);

        assert!(!field_def.has_default());

        let default_value = "true".to_owned();
        field_def.set_default(default_value.clone());

        assert!(field_def.has_default());
        assert_eq!(field_def.get_default(), default_value);
    }

    #[test]
    #[should_panic(expected = "get_default called on a field without a default value")]
    fn test_field_definition_get_default_without_default() {
        let field_type = PsqlFieldType::TEXT;
        let mandatory = false;
        let field_def = FieldDefinition::new_internal(field_type, mandatory);

        field_def.get_default();
    }

    #[test]
    fn test_field_definition_get_default_with_default() {
        let field_type = PsqlFieldType::FLOAT4;
        let mandatory = false;
        let default_value = "3.14".to_owned();
        let mut field_def = FieldDefinition::new(field_type, mandatory);
        field_def.set_default(default_value.clone());

        assert_eq!(field_def.get_default(), default_value);
    }

    // ResourceDefinition tests
    #[test]
    fn test_resource_definition_get_psql_table() {
        let psql_table = "my_table".to_owned();
        let resource_def = ResourceDefinition {
            psql_table: psql_table.clone(),
            psql_id_cols: Vec::new(),
            fields: HashMap::new(),
        };

        assert_eq!(resource_def.get_psql_table(), psql_table);
    }

    #[test]
    fn test_resource_definition_get_psql_id_cols() {
        let psql_id_cols = vec!["id".to_owned(), "name".to_owned()];
        let resource_def = ResourceDefinition {
            psql_table: String::new(),
            psql_id_cols: psql_id_cols.clone(),
            fields: HashMap::new(),
        };

        assert_eq!(resource_def.get_psql_id_cols(), psql_id_cols);
    }

    #[test]
    fn test_resource_definition_has_field() {
        let field_name = "field1";
        let field_def = FieldDefinition::new(PsqlFieldType::TEXT, true);

        let mut fields = HashMap::new();
        fields.insert(field_name.to_owned(), field_def);

        let resource_def = ResourceDefinition {
            psql_table: String::new(),
            psql_id_cols: Vec::new(),
            fields,
        };

        assert!(resource_def.has_field(field_name));
        assert!(!resource_def.has_field("nonexistent_field"));
    }

    #[test]
    fn test_resource_definition_try_get_field() {
        let field_name = "field1";
        let field_def = FieldDefinition::new(PsqlFieldType::TEXT, true);

        let mut fields = HashMap::new();
        fields.insert(field_name.to_owned(), field_def.clone());

        let resource_def = ResourceDefinition {
            psql_table: String::from("test"),
            psql_id_cols: vec![String::from("test_id")],
            fields,
        };

        let result = resource_def.try_get_field(field_name);
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), _field_def));

        let result = resource_def.try_get_field("nonexistent_field");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "error: Tried to get field [nonexistent_field] for table [{}], but the field does not exist.", resource_def.get_psql_table()
            )
        );
    }
}
