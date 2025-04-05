use crate::types::{Field, Type};
use crate::{DbcDescription, DbcVersion, Objects, Writer};
use heck::ToSnakeCase;
use std::collections::{BTreeMap, BTreeSet};

mod main_ty;
mod sqlite_converter;

pub use sqlite_converter::sqlite_converter;

fn not_pascal_case_name(s: &str) -> bool {
    s.contains('_')
}

pub fn create_table(d: &DbcDescription, o: &Objects, version: DbcVersion) -> Writer {
    let mut s = Writer::new(d.name());

    includes(&mut s, d, o, version);

    main_ty::create_main_ty(&mut s, d, o, version);

    create_row(&mut s, d, o);


    create_test(&mut s, d, version);

    s
}

fn insert(
    map: &mut BTreeMap<String, BTreeSet<String>>,
    module: impl ToString,
    name: impl ToString,
) {
    let module = module.to_string();
    let name = name.to_string();

    if let Some(v) = map.get_mut(&module) {
        v.insert(name);
    } else {
        let mut set = BTreeSet::new();
        set.insert(name);

        map.insert(module, set);
    }
}

fn includes(s: &mut Writer, d: &DbcDescription, o: &Objects, version: DbcVersion) {
    let mut map = BTreeMap::new();

    insert(&mut map, "std::io", "Write");

    insert(&mut map, "crate::header", "HEADER_SIZE");
    insert(&mut map, "crate::header", "DbcHeader");
    insert(&mut map, "crate::header", "parse_header");
    insert(&mut map, "crate::util", "StringCache");

    insert(&mut map, "crate", "DbcTable");
    insert(&mut map, "crate", "DbcRow");

    insert(&mut map, "super", format!("{}Table", version.to_str_capitalized()));

    if d.primary_key().is_some() {
        insert(&mut map, "crate", "Indexable");
    }

    if d.contains_localized_string() {
        insert(&mut map, "crate", "LocalizedString");
    }

    if d.contains_extended_localized_string() {
        insert(&mut map, "crate", "ExtendedLocalizedString");
    }

    if d.contains_localized_string() || d.contains_extended_localized_string() {
        insert(&mut map, "crate::tys", "WritableString");
    }

    let include_path = version.module_name();
    for foreign_key in d.foreign_keys() {
        if foreign_key == d.name() || !o.table_exists(foreign_key) {
            continue;
        }

        let name = foreign_key.to_snake_case();

        // import the foreign table
        insert(
            &mut map,
            format!("crate::{include_path}::{name}"),
            format!("{}", foreign_key),
        );

        // import the foreign key type
        insert(
            &mut map,
            format!("crate::{include_path}::{name}"),
            format!("{}Key", foreign_key),
        );

        // import indexable trait so we can query foreign keys
        insert(
            &mut map,
            "crate",
            "Indexable",
        );
    }

    let module = version.to_str();
    let base_import_path = format!("wow_world_base::{module}");

    for field in d.fields() {
        match field.ty() {
            Type::Array(array) => match array.ty() {
                Type::Enum(e) | Type::Flag(e) => {
                    insert(&mut map, base_import_path.clone(), e.name())
                }
                _ => {}
            },
            Type::Enum(e) | Type::Flag(e) => insert(&mut map, base_import_path.clone(), e.name()),
            _ => {}
        }
    }

    for (module, names) in map {
        if names.len() > 1 {
            s.wln(format!("use {module}::{{"));
            s.inc_indent();

            for (i, name) in names.iter().enumerate() {
                if i != 0 {
                    s.space();
                }
                s.w_break_at(format!("{name},"));
            }
            s.newline();

            s.closing_curly_with(";"); // use module
        } else {
            let name = names.first().unwrap();
            s.wln(format!("use {module}::{name};"))
        }
    }

    s.newline();
}

fn print_derives(s: &mut Writer, fields: &[Field], derive_copy: bool) {
    s.w("#[derive(Debug, Clone");
    if can_derive_copy(fields) && derive_copy {
        s.w_no_indent(", Copy");
    }

    s.w(", PartialEq");
    if can_derive_eq(fields) {
        s.w_no_indent(", Eq");
    }

    s.w(", PartialOrd");
    if can_derive_ord(fields) {
        s.w(", Ord");
    }

    if can_derive_hash(fields) {
        s.w(", Hash");
    }

    s.wln_no_indent(")]");

    // add optional derive for serde
    s.wln("#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]");
}

fn can_derive_copy(fields: &[Field]) -> bool {
    for field in fields {
        match field.ty() {
            Type::ExtendedStringRefLoc | Type::StringRefLoc | Type::StringRef => return false,
            Type::Array(array) => {
                if matches!(
                    array.ty(),
                    Type::ExtendedStringRefLoc | Type::StringRefLoc | Type::StringRef
                ) {
                    return false;
                }
            }
            _ => {}
        }
    }

    true
}
fn can_derive_hash(fields: &[Field]) -> bool {
    does_not_contain_float(fields)
}
fn can_derive_ord(fields: &[Field]) -> bool {
    does_not_contain_float(fields)
}
fn can_derive_eq(fields: &[Field]) -> bool {
    does_not_contain_float(fields)
}

fn does_not_contain_float(fields: &[Field]) -> bool {
    for field in fields {
        match field.ty() {
            Type::Float => return false,
            Type::Array(array) => {
                if matches!(array.ty(), Type::Float) {
                    return false;
                }
            }
            _ => {}
        }
    }

    true
}

fn create_row(s: &mut Writer, d: &DbcDescription, o: &Objects) {
    if not_pascal_case_name(d.name()) {
        s.wln("#[allow(non_camel_case_types)]");
    }

    print_derives(s, d.fields(), true);

    s.new_struct(format!("{}Row", d.name()), |s| {
        for field in d.fields() {
            let name = field.name();

            let ty = if let Type::ForeignKey { table, ty } = field.ty() {
                if o.table_exists(table) {
                    field.ty().rust_str()
                } else {
                    ty.rust_str()
                }
            } else {
                field.ty().rust_str()
            };

            s.wln(format!("pub {name}: {ty},"));
        }
    });

    // impl DbcRow for row struct
    s.bodyn(format!("impl DbcRow for {}Row", d.name()), |s| {
        // TODO: implement access to row fields
    });
}

fn print_field_comment(s: &mut Writer, field: &Field) {
    s.wln(format!("// {}: {}", field.name(), field.ty().str()));
}

fn create_test(s: &mut Writer, d: &DbcDescription, version: DbcVersion) {
    if d.name() == "CharacterCreateCameras"
        || d.name() == "SoundCharacterMacroLines"
        || d.name() == "SpellAuraNames"
        || d.name() == "SpellEffectNames"
    {
        // These do not have Dbcs
        return;
    }

    // some files are just empty in a default dbc set
    if match version {
        DbcVersion::Vanilla => false,
        DbcVersion::Tbc => false,
        DbcVersion::Wrath => {
            d.name() == "CharVariations"
        }
    }
    {
        return;
    }

    s.wln("#[cfg(test)]");
    s.open_curly("mod test");
    s.wln("use super::*;");
    s.wln("use std::fs::File;");
    s.wln("use std::io::Read;");
    s.newline();

    s.wln("#[test]");
    s.wln("#[ignore = \"requires DBC files\"]");
    s.open_curly(format!("fn {name}()", name = d.name().to_snake_case()));

    let ty = d.name();

    let test_dir_name = version.test_dir_name();

    // need to ascent once to get to the workspace root where the DBC files are
    s.wln(format!(
        "let mut file = File::open(\"../{test_dir_name}/{ty}.dbc\").expect(\"Failed to open DBC file\");",
    ));
    s.wln("let mut contents = Vec::new();");
    s.wln("file.read_to_end(&mut contents).expect(\"Failed to read DBC file\");");

    s.wln(format!(
        "let actual = {ty}::read(&mut contents.as_slice()).unwrap();",
    ));
    s.wln("let mut v = Vec::with_capacity(contents.len());");
    s.wln("actual.write(&mut v).unwrap();");

    s.wln(format!("let new = {ty}::read(&mut v.as_slice()).unwrap();"));
    s.wln("assert_eq!(actual, new);");

    s.closing_curly(); // fn
    s.closing_curly(); // mod test
}
